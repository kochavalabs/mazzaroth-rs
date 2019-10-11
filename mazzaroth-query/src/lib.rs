#[macro_use]
extern crate xdr_rs_serialize_derive;

mod xdr;

#[cfg(test)]
use xdr_rs_serialize::ser::XDROut;

#[cfg(test)]
use xdr_rs_serialize::de::XDRIn;

#[macro_export]
macro_rules! ops {
    (filter($left:ident == $right:ident)) => {{
        vec![xdr::Op::FILTER(xdr::Filter {
            expression: xdr::BooleanExpr {
                root: xdr::BoolNode {
                    value: xdr::BoolType::OP(xdr::BinaryOperator::EQUAL),
                    left: vec![xdr::BoolNode {
                        value: xdr::BoolType::IDENT(stringify!($left).to_string()),
                        left: vec![],
                        right: vec![],
                    }],
                    right: vec![xdr::BoolNode {
                        value: xdr::BoolType::IDENT(stringify!($right).to_string()),
                        left: vec![],
                        right: vec![],
                    }],
                },
            },
        })]
    }};
}

#[macro_export]
macro_rules! insert {
    ($table:ident, $value:ident) => {{
        let mut insert_value = Vec::new();
        $value.write_xdr(&mut insert_value).unwrap();
        xdr::Insert {
            table: stringify!($table).to_string(),
            data: insert_value,
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_basic_eq() {
        let result = ops!(filter(left_hand == right_hand));
        let expected = vec![xdr::Op::FILTER(xdr::Filter {
            expression: xdr::BooleanExpr {
                root: xdr::BoolNode {
                    value: xdr::BoolType::OP(xdr::BinaryOperator::EQUAL),
                    left: vec![xdr::BoolNode {
                        value: xdr::BoolType::IDENT("left_hand".to_string()),
                        left: vec![],
                        right: vec![],
                    }],
                    right: vec![xdr::BoolNode {
                        value: xdr::BoolType::IDENT("right_hand".to_string()),
                        left: vec![],
                        right: vec![],
                    }],
                },
            },
        })];
        assert_eq!(expected, result);
    }

    #[test]
    fn insert_macro() {
        let value = xdr::Select::default();
        let result = insert!(CoolTable, value);
        assert_eq!(
            xdr::Insert {
                table: "CoolTable".to_string(),
                data: vec![0, 0, 0, 0],
            },
            result
        );
    }
}
