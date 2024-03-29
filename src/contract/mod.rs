use std::fmt;
use xdr_rs_serialize::error::Error;

/// This trait defines the execute function that can be called on a contract.
/// The implementation is generated by the derive macro but this trait must
/// be included by the contract implementation.
pub trait ContractInterface {
    fn execute(&mut self, payload: &[u8]) -> Result<Vec<u8>, ContractError>;
}

#[derive(Debug)]
pub enum ContractErrorKinds {
    DeserializeError(Error),
    InvalidArguments,
    InvalidFunctionName,
}

#[derive(Debug)]
pub struct ContractError {
    kind: ContractErrorKinds,
}

impl ContractError {
    fn from_kind(kind: ContractErrorKinds) -> Self {
        ContractError { kind }
    }

    fn kind(&self) -> &ContractErrorKinds {
        &self.kind
    }

    pub fn invalid_arguments() -> Self {
        ContractError {
            kind: ContractErrorKinds::InvalidArguments,
        }
    }

    pub fn invalid_function() -> Self {
        ContractError {
            kind: ContractErrorKinds::InvalidFunctionName,
        }
    }
}

impl std::fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            ContractErrorKinds::DeserializeError(err) => {
                write!(f, "Error deserializing arguments: {}", err)
            }
            ContractErrorKinds::InvalidArguments => {
                write!(f, "Failed to parse arguments for function.")
            }
            ContractErrorKinds::InvalidFunctionName => {
                write!(f, "Could not find function with given name.")
            }
        }
    }
}

impl From<Error> for ContractError {
    fn from(deserialize_err: Error) -> Self {
        ContractError::from_kind(ContractErrorKinds::DeserializeError(deserialize_err))
    }
}
