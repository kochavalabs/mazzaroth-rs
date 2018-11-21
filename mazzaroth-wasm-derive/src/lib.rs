#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::Span;

mod contract;
use contract::{Contract,TraitItem};

mod error;
use error::{Result,ProcError};


#[proc_macro_attribute]
pub fn mazzaroth_abi(args: TokenStream, input: TokenStream) -> TokenStream {
	let args_toks = parse_macro_input!(args as syn::AttributeArgs);
	let input_toks = parse_macro_input!(input as syn::Item);

	let output = match impl_mazzaroth_abi(args_toks, input_toks) {
		Ok(output) => output,
		Err(err) => panic!("roth_abi encountered error: {}", err),
	};

	output.into()
}

fn impl_mazzaroth_abi(args: syn::AttributeArgs, input: syn::Item) -> Result<proc_macro2::TokenStream> {
	// Get the name for the generated Contract from the Arg
	if args.len() == 0 || args.len() > 1 {
		return Err(ProcError::invalid_arguments(args.len()));
	}

	// Get the contract name passed as an argument to the roth_abi macro
	let argument_name = 
		if let syn::NestedMeta::Meta(syn::Meta::Word(ident)) = args.get(0).unwrap() {
			Ok(ident.to_string())
		} else {
			Err(ProcError::malformed_argument())
		}?;
	let argument_ident = syn::Ident::new(&argument_name, Span::call_site());

	let contract = Contract::from_item(input);
	let contract_ident = syn::Ident::new(contract.name(), Span::call_site());

	// Mod that is created around contract trait
	let mod_name = format!("roth_abi_impl_{}", &contract.name().clone());
	let mod_name_ident = syn::Ident::new(&mod_name, Span::call_site());

	// Tokenize the contract which will have a single entry
	// to call the contract functions
	let contract_toks = tokenize_contract(&argument_name, &contract);

	Ok(quote! {
		#contract // Automatically calls the quote::ToTokens function
		mod #mod_name_ident {
			use super::#contract_ident; // Provide access to the user contract
			#contract_toks
		}
		pub use self::#mod_name_ident::#argument_ident;
	})
}

fn tokenize_contract(name: &str, contract: &Contract) -> proc_macro2::TokenStream {

	// Loop through the trait items of the contract and for Functions build a 
	// quote map of function name to a function wrapper that gets arguments from encoded bytes
	// and returns bytes
	let functions: Vec<proc_macro2::TokenStream> = contract.trait_items().iter().filter_map(|item| {
		match *item {
			TraitItem::Function(ref function) => {
				let function_ident = &function.name;
				let arg_types = &function.arguments.iter().map(|&(_, ref ty)| quote! { #ty });

				if function.ret_types.is_empty() {
					Some(quote! {
						#function_ident => {
							let mut decoder = mazzaroth_wasm::Decoder::new(payload);
							inner.#function_ident(
								#(decoder.pop::<#arg_types>().expect("argument decoding failed")),*
							);
							Vec::new()
						}
					})
				} else {
					Some(quote! {
						#function_ident => {
							let mut decoder = mazzaroth_wasm::Decoder::new(payload);
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
			_ => None,
		}
	}).collect();

	let endpoint_ident = syn::Ident::new(name, Span::call_site());
	let name_ident = syn::Ident::new(&contract.name(), Span::call_site());

	quote!{
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

		impl<T: #name_ident> pwasm_abi::eth::EndpointInterface for #endpoint_ident<T> {
			#[allow(unused_mut)]
			#[allow(unused_variables)]
			fn execute(&mut self, payload: &[u8]) -> Vec<u8> {
				let inner = &mut self.inner;
				if payload.len() < 4 {
					panic!("Invalid abi invoke");
				}
				let method_id = ((payload[0] as u32) << 24)
					+ ((payload[1] as u32) << 16)
					+ ((payload[2] as u32) << 8)
					+ (payload[3] as u32);

				let method_payload = &payload[4..];

				match method_id {
					#(#functions,)*
					_ => panic!("Invalid method signature"),
				}
			}
		}
	}
}