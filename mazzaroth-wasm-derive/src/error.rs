use std::fmt;

use json::JsonError;

/// The result type
pub type Result<T> = std::result::Result<T, ProcError>;

#[derive(Debug)]
pub struct ProcError {
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    /// An error that occured upon a JSON operation.
    JsonError(JsonError),

    InvalidArguments {
        found: usize,
    },

    MalformedArgument,
}

impl ProcError {
    fn from_kind(kind: ErrorKind) -> Self {
        ProcError { kind }
    }

    fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn invalid_arguments(found: usize) -> Self {
        assert!(found != 1);
        ProcError {
            kind: ErrorKind::InvalidArguments { found },
        }
    }

    pub fn malformed_argument() -> Self {
        ProcError {
            kind: ErrorKind::MalformedArgument,
        }
    }
}

impl std::fmt::Display for ProcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            ErrorKind::JsonError(err) => write!(f, "{}", err),
            ErrorKind::InvalidArguments { found } => {
                write!(f, "expected 1 argument but found {}", found)
            }
            ErrorKind::MalformedArgument { .. } => {
                write!(f, "malformed argument passed to roth_abi")
            }
        }
    }
}

impl std::error::Error for ProcError {
    fn description(&self) -> &str {
        match self.kind() {
            ErrorKind::JsonError(err) => err.description(),
            ErrorKind::InvalidArguments { .. } => "did not find exactly one argument to roth_abi",
            ErrorKind::MalformedArgument { .. } => "malformed argument passed to roth_abi",
        }
    }
}

impl From<JsonError> for ProcError {
    fn from(json_err: JsonError) -> Self {
        ProcError::from_kind(ErrorKind::JsonError(json_err))
    }
}
