//! # Mazzaroth WASM Derive Library
//!
//! The Mazzaroth WASM Derive Library is a rust library that defines the macros
//! used to compile Mazzaroth Smart Contracts and generate the JSON ABI.
//!
//! ## How to use
//!
//! The first step to using this library is to include the necessary dependencies.
//! The following 3 dependencies should be included in your Cargo.toml:
//!
//! mazzaroth-wasm
//! mazzaroth-wasm-derive
//! mazzaroth-wasm-xdr
//!
//! Every contract will have a similar base layout for the main function and the contract trait definition.
//! `main()` is used as the entrypoint and has several important features.  It will instantiate the contract,
//! call a host function to retrieve function input, execute the function, and return a response.
//!
//! Here is a basic Hello World contract example:
//! ```
//! // must include the ContractInterface and mazzaroth_abi for compiling the macro
//! extern crate mazzaroth_wasm;
//! extern crate mazzaroth_wasm_derive;
//! use mazzaroth_wasm::ContractInterface;
//! use mazzaroth_wasm_derive::mazzaroth_abi;
//!
//! // using specific external host modules
//! use mazzaroth_wasm::external::{transaction, log};
//!
//! #[no_mangle]
//! pub fn main() {
//!     // panic hook is set to call the host error log function when a panic occurs
//!     std::panic::set_hook(Box::new(mazzaroth_wasm::external::errors::hook));
//!
//!     // Creates a new instance of the ABI generated around the Contract
//!     let mut contract = HelloWorld::new(Hello {});
//!
//!     // Use a host function to get arguments
//!     let args = transaction::arguments();
//!
//!     // Execute calls one of the functions defined in the contract
//!     // Input for the function to call and it's params comes from the Runtime
//!     let response = contract.execute(&args);
//!
//!     // Provide return value through host call
//!     transaction::ret(response);
//! }
//!
//! // mazzaroth_abi used to generate the contract from the trait during compilation
//! #[mazzaroth_abi(HelloWorld)]
//! pub trait HelloWorldContract {
//!     // hello() defined as a readonly function
//!     #[readonly]
//!     fn hello(&mut self) -> u32;
//! }
//!
//! // Struct used to implement the contract trait
//! pub struct Hello {}
//!
//! // Actual contract implementation
//! impl HelloWorldContract for Hello {
//!     fn hello(&mut self) -> u32 {
//!         log("Hello World!".to_string());
//!         14
//!     }
//! }
//! ```
#![recursion_limit = "256"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use proc_macro::TokenStream;
use proc_macro2::Span;

mod contract;
use contract::{Contract, TraitItem};

mod error;
use error::{ProcError, Result};

mod json;
use json::write_json_abi;

/// Macro used to mark the trait that defines the mazzaroth contract
///
/// The argument becomes the module name used to construct the contract in main.
///
/// Example:
/// ```
/// #[mazzaroth_abi(HelloWorld)]
/// pub trait HelloWorldContract {
///     
/// }
/// ```
#[proc_macro_attribute]
pub fn mazzaroth_abi(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_toks = parse_macro_input!(args as syn::AttributeArgs);
    let input_toks = parse_macro_input!(input as syn::Item);

    let output = match impl_mazzaroth_abi(args_toks, input_toks) {
        Ok(output) => output,
        Err(err) => panic!("mazzaroth_abi encountered error: {}", err),
    };

    output.into()
}

fn impl_mazzaroth_abi(
    args: syn::AttributeArgs,
    input: syn::Item,
) -> Result<proc_macro2::TokenStream> {
    // Get the name for the generated Contract from the Arg
    if args.len() == 0 || args.len() > 1 {
        return Err(ProcError::invalid_arguments(args.len()));
    }

    // Get the contract name passed as an argument to the mazzaroth_abi macro
    let argument_name = if let syn::NestedMeta::Meta(syn::Meta::Word(ident)) = args.get(0).unwrap()
    {
        Ok(ident.to_string())
    } else {
        Err(ProcError::malformed_argument())
    }?;
    let argument_ident = syn::Ident::new(&argument_name, Span::call_site());

    let contract = Contract::from_item(input);

    // Write out a json abi for the functions available
    write_json_abi(&contract)?;

    // Mod that is created around contract trait
    let mod_name = format!("mazzaroth_abi_impl_{}", &contract.name().clone());
    let mod_name_ident = syn::Ident::new(&mod_name, Span::call_site());

    // Tokenize the contract which will have a single entry
    // to call the contract functions
    let contract_toks = tokenize_contract(&argument_name, &contract);

    // Note: Imports are included in the generated module here
    // So if types are added that can be used as function params or returns, they must be included.
    Ok(quote! {
        #contract
        mod #mod_name_ident {
            extern crate mazzaroth_wasm;
            extern crate mazzaroth_xdr;
            use super::*; // Provide access to the user contract
            #contract_toks
        }
        pub use self::#mod_name_ident::#argument_ident;
    })
}

// Tokenize contract to an implementation with a callable execute function
// TODO: Insert owenrship check for constructor?
fn tokenize_contract(name: &str, contract: &Contract) -> proc_macro2::TokenStream {
    let constructor = contract.constructor().map(|signature| {
        let arg_types = signature
            .arguments
            .iter()
            .map(|&(_, ref ty)| quote! { #ty });
        quote! {
            self.inner.constructor(
                #(decoder.pop::<#arg_types>().expect("argument decoding failed")),*
            );
        }
    });

    // Loop through the trait items of the contract and for Functions build a
    // quote map of function name to a function wrapper that gets arguments from encoded bytes
    // and returns bytes. Also includes Readonly functions in contract.
    let functions: Vec<proc_macro2::TokenStream> = contract.trait_items().iter().filter_map(|item| {
		match *item {
			TraitItem::Function(ref function) => {
				let function_ident = &function.name;

				// Don't include constructor in callable function list
				match function_ident.to_string().as_ref() {
					"constructor" => None,
					_ => {
						// Create a matchname string literal that matches name of function
						let match_name = syn::Lit::Str(syn::LitStr::new(&function_ident.to_string(), Span::call_site()));

						let arg_types = function.arguments.iter().map(|&(_, ref ty)| quote! { #ty });

						if function.ret_types.is_empty() {
							Some(quote! {
								#match_name => {
									inner.#function_ident(
										#(decoder.pop::<#arg_types>().expect("argument decoding failed")),*
									);
									Vec::new()
								}
							})
						} else {
							Some(quote! {
								#match_name => {
									let result = inner.#function_ident(
										#(decoder.pop::<#arg_types>().expect("argument decoding failed")),*
									);
									let mut encoder = mazzaroth_wasm::Encoder::new();
									encoder.push(result);
									encoder.values()
								}
							})
						}
					},
				}
			},
			_ => None,
		}
	}).collect();

    // Same as above but only for Readonly functions
    let readonly_functions: Vec<proc_macro2::TokenStream> = contract.trait_items().iter().filter_map(|item| {
		match *item {
			TraitItem::Readonly(ref function) => {
				let function_ident = &function.name;

				// Don't include constructor in callable function list
				match function_ident.to_string().as_ref() {
					"constructor" => None,
					_ => {

						// Create a matchname string literal that matches name of function
						let match_name = syn::Lit::Str(syn::LitStr::new(&function_ident.to_string(), Span::call_site()));

						let arg_types = function.arguments.iter().map(|&(_, ref ty)| quote! { #ty });

						if function.ret_types.is_empty() {
							Some(quote! {
								#match_name => {
									inner.#function_ident(
										#(decoder.pop::<#arg_types>().expect("argument decoding failed")),*
									);
									Vec::new()
								}
							})
						} else {
							Some(quote! {
								#match_name => {
									let result = inner.#function_ident(
										#(decoder.pop::<#arg_types>().expect("argument decoding failed")),*
									);
									let mut encoder = mazzaroth_wasm::Encoder::new();
									encoder.push(result);
									encoder.values()
								}
							})
						}
					},
				}
			},
			_ => None,
		}
	}).collect();

    let endpoint_ident = syn::Ident::new(name, Span::call_site());
    let name_ident = syn::Ident::new(&contract.name(), Span::call_site());

    quote! {
        pub struct #endpoint_ident<T: #name_ident> {
            pub inner: T,
        }

        impl<T: #name_ident> From<T> for #endpoint_ident<T> {
            fn from(inner: T) -> #endpoint_ident<T> {
                #endpoint_ident {
                    inner: inner,
                }
            }
        }

        impl<T: #name_ident> #endpoint_ident<T> {
            pub fn new(inner: T) -> Self {
                #endpoint_ident {
                    inner: inner,
                }
            }

            pub fn instance(&self) -> &T {
                &self.inner
            }
        }

        impl<T: #name_ident> mazzaroth_wasm::ContractInterface for #endpoint_ident<T> {
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            fn execute(&mut self, payload: &[u8]) -> Vec<u8> {
                let inner = &mut self.inner;

                // first decode the input from stream
                let mut payload_decoder = mazzaroth_wasm::Decoder::new(payload);
                let mut input = payload_decoder.pop::<mazzaroth_xdr::Input>().expect("argument decoding failed");

                // Then create a decoder for params
                let mut decoder = mazzaroth_wasm::InputDecoder::new(&input.parameters);

                match input.inputType {
                    mazzaroth_xdr::InputType::EXECUTE => {
                        // Call executes a normal contract function (excludes readonly functions)
                        match input.function.as_str() {
                            #(#functions,)*
                            _ => panic!("Invalid non-readonly method name"),
                        }
                    },
                    mazzaroth_xdr::InputType::READONLY => {
                        // Readonly executes a function tagged with readonly
                        // First param should be the string function name to call
                        match input.function.as_str() {
                            #(#readonly_functions,)*
                            _ => panic!("Invalid readonly method name"),
                        }
                    },
                    mazzaroth_xdr::InputType::CONSTRUCTOR => {
                        // Call the constructor with dynamic params
                        #constructor
                        Vec::new()
                    },
                    _ => panic!("Invalid input type"),
                }
            }
        }
    }
}
