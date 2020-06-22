use proc_macro2::Span;
use quote;
use quote::TokenStreamExt; // Provides access to append_all for TokenStream
use std::collections::HashMap;
use syn;
use syn::Ident;
use syn::Meta::{List, NameValue};
use syn::NestedMeta::{Literal, Meta};

/// Contract is built from items passed to procedural macro
/// and updated
pub struct Contract {
    /// Name of the contract trait.
    name: String,

    /// Either a method defined in the trait
    /// or other TraitItem that is ignored
    trait_items: Vec<TraitItem>,
}

/// Represents a function defined in the contract trait.
/// Can use this to get info about the Args and Returns for each function
#[derive(Clone)]
pub struct Function {
    /// Name of the function.
    pub name: syn::Ident,
    // Arg information for this function.
    pub method_sig: syn::MethodSig,
    // Parsed args with their type
    pub arguments: Vec<(syn::Pat, syn::Type)>,
    // Return types for the function.
    pub ret_types: Vec<syn::Type>,

    /// Codecs defined by tag with their encoding type
    pub codec: HashMap<String, String>,
}

/// Item within the trait, function or Readonly function
/// Other can be const, type, macro, or verbatim
pub enum TraitItem {
    Function(Function),
    Readonly(Function),
    Other(syn::TraitItem),
}

impl Contract {
    pub fn from_item(contract_item: syn::Item) -> Self {
        let contract_trait = match contract_item {
            syn::Item::Trait(item_trait) => item_trait,
            _ => panic!("Contract macro only works with trait declarations!"),
        };

        // Parse the trait items
        let items = contract_trait.items.into_iter().map(TraitItem::from_contract_item).collect();

        Contract {
            name: contract_trait.ident.to_string(),
            trait_items: items,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn trait_items(&self) -> &[TraitItem] {
        &self.trait_items
    }
}

impl TraitItem {
    /// Takes a TraitItem from the Contract trait and returns it as our own
    /// TraitItem, either a Function or Other.
    pub fn from_contract_item(item: syn::TraitItem) -> Self {
        match item {
            syn::TraitItem::Method(method_trait_item) => {
                if method_trait_item.default.is_some() {
                    return TraitItem::Other(syn::TraitItem::Method(method_trait_item));
                }

                // Look for codec attrs
                let codec = parse_attribute_codec(&method_trait_item.attrs);

                // If the function has the readonly attribute return as a Readonly function, else it is a regular function
                if has_attribute(&method_trait_item.attrs, "readonly") {
                    TraitItem::Readonly(new_function(
                        method_trait_item.sig.ident.clone(),
                        method_trait_item.sig,
                        codec,
                    ))
                } else {
                    // Returns the TraitItem as a new Function
                    TraitItem::Function(new_function(
                        method_trait_item.sig.ident.clone(),
                        method_trait_item.sig,
                        codec,
                    ))
                }
            }
            trait_item => TraitItem::Other(trait_item),
        }
    }
}

fn has_attribute(attrs: &[syn::Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| {
        if let Some(first_seg) = attr.path.segments.first() {
            return first_seg.value().ident == name;
        };
        false
    })
}

fn parse_attribute_codec(attrs: &[syn::Attribute]) -> HashMap<String, String> {
    let mut codec = HashMap::new();
    for meta_items in attrs.iter().filter_map(get_mazzaroth_meta_items) {
        for meta_item in meta_items {
            match meta_item {
                // Parse `#[mazzaroth_codec({key1} = "{value1}", {key2} = "{value2}"))]`
                Meta(NameValue(ref m)) => {
                    let s = get_lit_str(&m.ident, &m.lit);
                    // println!("Name of key {}, value {}", m.ident, s.value());
                    codec.insert(m.ident.to_string(), s.value());
                }
                Meta(ref meta_item) => {
                    panic!(
                        "unknown variant in mazzaroth_codec attribute `{}`",
                        meta_item.name()
                    );
                }

                Literal(ref _lit) => {
                    panic!("unexpected literal in mazzaroth_codec attribute");
                }
            }
        }
    }
    codec
}

fn get_mazzaroth_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "mazzaroth_codec" {
        match attr.interpret_meta() {
            Some(List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
            _ => {
                // TODO: produce an error?
                None
            }
        }
    } else {
        None
    }
}

fn get_lit_str<'a>(attr_name: &Ident, lit: &'a syn::Lit) -> &'a syn::LitStr {
    if let syn::Lit::Str(ref lit) = *lit {
        lit
    } else {
        panic!(
            "expected mazzaroth_codec attribute to be a string: `{} = \"...\"`",
            attr_name
        )
    }
}

fn new_function(
    name: syn::Ident,
    method_sig: syn::MethodSig,
    codec: HashMap<String, String>,
) -> Function {
    // Get arguments from method sig
    let arguments: Vec<(syn::Pat, syn::Type)> = iter_signature(&method_sig).collect();

    // Create the vector of return types from method sig
    let ret_types: Vec<syn::Type> = match method_sig.decl.output.clone() {
        syn::ReturnType::Default => Vec::new(),
        syn::ReturnType::Type(_, ty) => match *ty {
            syn::Type::Tuple(tuple_type) => tuple_type.elems.into_iter().collect(),
            ty => vec![ty],
        },
    };

    Function {
        name: name,
        method_sig: method_sig,
        arguments: arguments,
        ret_types: ret_types,
        codec: codec,
    }
}

/// Iterates the arguments of a functions
pub struct SignatureIterator<'a> {
    method_sig: &'a syn::MethodSig,
    position: usize,
}

impl<'a> Iterator for SignatureIterator<'a> {
    type Item = (syn::Pat, syn::Type);

    fn next(&mut self) -> Option<Self::Item> {
        while self.position < self.method_sig.decl.inputs.len() {
            if let syn::FnArg::Captured(ref arg_captured) =
                self.method_sig.decl.inputs[self.position]
            {
                self.position += 1;
                return Some((arg_captured.pat.clone(), arg_captured.ty.clone()));
            } else {
                self.position += 1;
            }
        }
        None
    }
}

pub fn iter_signature(method_sig: &syn::MethodSig) -> SignatureIterator {
    SignatureIterator {
        method_sig: method_sig,
        position: 0,
    }
}

impl quote::ToTokens for Contract {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // TODO: Store ident instead of name in Contract?
        let trait_ident = syn::Ident::new(&self.name, Span::call_site());
        let items = &self.trait_items;

        // Put items in contract
        tokens.append_all(quote! (
            pub trait #trait_ident {
                #(#items)*
            }
        ));
    }
}

impl quote::ToTokens for TraitItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match *self {
            TraitItem::Readonly(ref function) => {
                tokens.append_all(
                    syn::TraitItem::Method(syn::TraitItemMethod {
                        attrs: Vec::new(),
                        sig: function.method_sig.clone(),
                        default: None,
                        semi_token: None,
                    })
                    .into_token_stream(),
                );
            }
            TraitItem::Function(ref function) => {
                tokens.append_all(
                    syn::TraitItem::Method(syn::TraitItemMethod {
                        attrs: Vec::new(),
                        sig: function.method_sig.clone(),
                        default: None,
                        semi_token: None,
                    })
                    .into_token_stream(),
                );
            }
            TraitItem::Other(ref item) => {
                tokens.append_all(&[item]);
            }
        }
    }
}
