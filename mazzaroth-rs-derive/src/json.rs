//! JSON generation

use xdr_rs_serialize::ser::XDROut;
use mazzaroth_xdr::{Abi, FunctionSignature, Parameter};

use contract;

use std;
use std::io;
use std::io::Write;

/// The result type for JSON errors.
pub type JsonResult<T> = std::result::Result<T, JsonError>;

/// Errors that may occur during JSON operations.
#[derive(Debug)]
pub enum JsonError {
    FailedToCreateDirectory(io::Error),
    FailedToCreateJsonFile(io::Error),
    FailedToWriteJsonAbiFile(io::Error),
    FailedToWriteJsonBytes(xdr_rs_serialize::error::Error),
}

impl JsonError {
    /// Returns a JSON error indicating that the creation of the
    /// directory that will contain the JSON file failed.
    pub fn failed_to_create_dir(err: io::Error) -> Self {
        JsonError::FailedToCreateDirectory(err)
    }

    /// Returns a JSON error indicating that the creation of the JSON
    /// abi file failed.
    pub fn failed_to_create_json_file(err: io::Error) -> Self {
        JsonError::FailedToCreateJsonFile(err)
    }

    /// Returns a JSON error indicating that the writing of the JSON
    /// to bytes failed.
    pub fn failed_to_write_json_bytes(err: xdr_rs_serialize::error::Error) -> Self {
        JsonError::FailedToWriteJsonBytes(err)
    }

    /// Returns a JSON error indicating that the writing of the JSON
    /// abi file failed.
    pub fn failed_to_write_json_abi_file(err: io::Error) -> Self {
        JsonError::FailedToWriteJsonAbiFile(err)
    }
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            JsonError::FailedToCreateDirectory(err) => {
                write!(f, "failed to create directory for JSON abi file: {:?}", err)
            }
            JsonError::FailedToCreateJsonFile(err) => {
                write!(f, "failed to create JSON abi file: {:?}", err)
            }
            JsonError::FailedToWriteJsonBytes(err) => {
                write!(f, "failed to write JSON bytes: {:?}", err)
            }
            JsonError::FailedToWriteJsonAbiFile(err) => {
                write!(f, "failed to write JSON abi file: {:?}", err)
            }
        }
    }
}

impl std::error::Error for JsonError {
    fn description(&self) -> &str {
        match self {
            JsonError::FailedToCreateDirectory(_) => {
                "failed to create directory for the JSON abi file"
            }
            JsonError::FailedToCreateJsonFile(_) => "failed to create JSON abi file",
            JsonError::FailedToWriteJsonBytes(_) => "failed to write JSON bytes",
            JsonError::FailedToWriteJsonAbiFile(_) => "failed to write JSON abi file",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            JsonError::FailedToCreateDirectory(err) => Some(err),
            JsonError::FailedToCreateJsonFile(err) => Some(err),
            JsonError::FailedToWriteJsonBytes(err) => Some(err),
            JsonError::FailedToWriteJsonAbiFile(err) => Some(err),
        }
    }
}

/// Writes generated abi JSON file to destination in default target directory.
pub fn write_json_abi(intf: &contract::Contract) -> JsonResult<()> {
    use std::{env, fs, path};

    let target = {
        let mut target =
            path::PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or(".".to_owned()));
        target.push("target");
        target.push("json");
        fs::create_dir_all(&target).map_err(|err| JsonError::failed_to_create_dir(err))?;
        target.push(&format!("{}.json", intf.name()));
        target
    };

    // Create the target/*.json file
    let mut f =
        fs::File::create(target).map_err(|err| JsonError::failed_to_create_json_file(err))?;

    // Convert the Contract into the Mazzaroth_xdr ABI object
    let abi: Abi = intf.into();

    // Serialize the ABI object to JSON bytes
    let mut val_bytes: Vec<u8> = Vec::new();
    abi.write_json(&mut val_bytes).map_err(|err| JsonError::failed_to_write_json_bytes(err))?;

    // Write JSON Bytes to the target file
    f.write_all(&val_bytes).map_err(|err| JsonError::failed_to_write_json_abi_file(err))?;

    Ok(())
}

// Trait implementation to build an ABI object from the Contract
impl<'a> From<&'a contract::Contract> for Abi {
    fn from(intf: &contract::Contract) -> Self {
        let mut result = Vec::new();
        for item in intf.trait_items() {
            match *item {
                // contract::Item::Event(ref event) => result.push(AbiEntry::Event(event.into())),
                contract::TraitItem::Function(ref signature) => {
                    let mut function: FunctionSignature = signature.into();
                    function.functionType = "function".to_string();
                    result.push(function)
                }
                contract::TraitItem::Readonly(ref signature) => {
                    let mut function: FunctionSignature = signature.into();
                    function.functionType = "readonly".to_string();
                    result.push(function)
                }
                _ => {}
            }
        }

        Abi{
            functions: result,
        }
    }
}

// Trait implementation to build a Function Signature from a Contract function
impl<'a> From<&'a contract::Function> for FunctionSignature {
    fn from(item: &contract::Function) -> Self {
       FunctionSignature {
            functionType: "".to_string(), // To be set by caller based on function type
            name: item.name.to_string(),
            inputs: item
                .arguments
                .iter()
                .map(|&(ref pat, ref ty)| Parameter {
                    name: quote! { #pat }.to_string(),
                    parameterType: canonicalize_type(ty),
                    codec: check_codec(item, ty),
                })
                .collect(),
            outputs: item
                .ret_types
                .iter()
                .enumerate()
                .map(|(idx, ty)| Parameter {
                    name: format!("returnValue{}", idx),
                    parameterType: canonicalize_type(ty),
                    codec: check_codec(item, ty),
                })
                .collect(),
        }
    }
}

// Return the codec value for the type, or "bytes"
fn check_codec(item: &contract::Function, ty: &syn::Type) -> String {
    if let Some(value) = item.codec.get(&canonicalize_type(ty)) {
        value.to_string()
    } else {
        "bytes".to_string()
    }
}

fn push_int_const_expr(target: &mut String, expr: &syn::Expr) {
    match expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(lit_int),
            ..
        }) => target.push_str(&format!("{}", lit_int.value())),
        _ => panic!("Cannot use something other than integer literal in this constant expression"),
    }
}

/// canonicalize vector as array or bytes if Vec<u8>
fn push_canonicalized_vec(target: &mut String, args: &syn::PathArguments) {
    match args {
        syn::PathArguments::AngleBracketed(gen_args) => {
            let last_arg = gen_args.args.last().unwrap();
            let last_type = last_arg.value();
            if let syn::GenericArgument::Type(syn::Type::Path(type_path)) = last_type {
                return if type_path.qself.is_none()
                    && type_path.path.segments.last().unwrap().value().ident == "u8"
                {
                    target.push_str("bytes");
                } else {
                    push_canonicalized_path(target, type_path);
                    target.push_str("[]");
                };
            }
            panic!("Unsupported generic arguments")
        }
        _ => panic!("Unsupported vec arguments"),
    }
}

fn push_canonicalized_primitive(target: &mut String, seg: &syn::PathSegment) {
    match seg.ident.to_string().as_str() {
        "u32" => target.push_str("uint32"),
        "i32" => target.push_str("int32"),
        "u64" => target.push_str("uint64"),
        "i64" => target.push_str("int64"),
        "String" => target.push_str("string"),
        "bool" => target.push_str("bool"),
        "Vec" => push_canonicalized_vec(target, &seg.arguments),
        val => target.push_str(val),
    }
}

fn push_canonicalized_path(target: &mut String, type_path: &syn::TypePath) {
    assert!(
        type_path.qself.is_none(),
        "Unsupported type path for canonicalization!"
    );
    let last_path = type_path.path.segments.last().unwrap();
    push_canonicalized_primitive(target, *last_path.value())
}

fn push_canonicalized_type(target: &mut String, ty: &syn::Type) {
    match ty {
        syn::Type::Path(type_path) if type_path.qself.is_none() => {
            push_canonicalized_path(target, &type_path)
        }
        syn::Type::Array(type_array) => {
            // Special cases for `bytesN`
            if let syn::Type::Path(type_path) = &*type_array.elem {
                if "u8"
                    == type_path
                        .path
                        .segments
                        .last()
                        .unwrap()
                        .value()
                        .ident
                        .to_string()
                {
                    target.push_str("bytes");
                    push_int_const_expr(target, &type_array.len);
                    return;
                }
            }

            panic!("Unsupported! Use variable-size arrays")
        }
        other_type => panic!(
            "[e2] Unable to handle param of type {:?}: not supported by abi",
            other_type
        ),
    }
}

/// Returns the canonicalized string representation for the given type.
pub fn canonicalize_type(ty: &syn::Type) -> String {
    let mut result = String::new();
    push_canonicalized_type(&mut result, ty);
    result
}
