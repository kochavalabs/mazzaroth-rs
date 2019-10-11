#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[allow(unused_imports)]
use xdr_rs_serialize::de::{
    read_fixed_array, read_fixed_opaque, read_var_array, read_var_opaque, read_var_string, XDRIn,
};
use xdr_rs_serialize::error::Error;
#[allow(unused_imports)]
use xdr_rs_serialize::ser::{
    write_fixed_array, write_fixed_opaque, write_var_array, write_var_opaque, write_var_string,
    XDROut,
};

// Namspace start ciq

// Start typedef section

#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct BinaryValue {
    #[array(var = 2147483647)]
    pub t: Vec<u8>,
}

// End typedef section

// Start struct section

#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct BoolNode {
    pub value: BoolType,

    #[array(var = 1)]
    pub left: Vec<BoolNode>,

    #[array(var = 1)]
    pub right: Vec<BoolNode>,
}

#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct BooleanExpr {
    pub root: BoolNode,
}

// End struct section

#[derive(PartialEq, Clone, Debug, XDROut, XDRIn)]
pub enum BinaryOperator {
    EQUAL = 0,
    LT = 1,
    GT = 2,
    OR = 3,
    AND = 4,
}

impl Default for BinaryOperator {
    fn default() -> Self {
        BinaryOperator::EQUAL
    }
}

#[derive(PartialEq, Clone, Debug, XDROut, XDRIn)]
pub enum BoolCategory {
    BOOL = 0,
    IDENT = 1,
    OP = 2,
    VALUE = 3,
}

impl Default for BoolCategory {
    fn default() -> Self {
        BoolCategory::BOOL
    }
}
// Start union section

#[derive(PartialEq, Clone, Debug, XDROut, XDRIn)]
pub enum BoolType {
    BOOL(bool),

    IDENT(String),

    OP(BinaryOperator),

    VALUE(BinaryValue),
}

impl Default for BoolType {
    fn default() -> Self {
        BoolType::BOOL(bool::default())
    }
}
// End union section

// Namspace end ciq
// Namspace start ciq

// Start typedef section

#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct ID {
    #[array(var = 2147483647)]
    pub t: String,
}

// End typedef section

// Start struct section

#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct Select {
    #[array(var = 2147483647)]
    pub props: Vec<ID>,
}

#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct Filter {
    pub expression: BooleanExpr,
}

#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct Insert {
    #[array(var = 2147483647)]
    pub table: String,

    #[array(var = 2147483647)]
    pub data: Vec<u8>,
}

// End struct section

#[derive(PartialEq, Clone, Debug, XDROut, XDRIn)]
pub enum OpType {
    SELECT = 0,
    FILTER = 1,
}

impl Default for OpType {
    fn default() -> Self {
        OpType::SELECT
    }
}
// Start union section

#[derive(PartialEq, Clone, Debug, XDROut, XDRIn)]
pub enum Op {
    SELECT(Select),

    FILTER(Filter),
}

impl Default for Op {
    fn default() -> Self {
        Op::SELECT(Select::default())
    }
}
// End union section

// Namspace end ciq
// Namspace start ciq

// Start typedef section

// End typedef section

// Start struct section

#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct Query {
    #[array(var = 64)]
    pub source: String,

    #[array(var = 2147483647)]
    pub operations: Vec<Op>,
}

// End struct section

// Start union section

// End union section

// Namspace end ciq
