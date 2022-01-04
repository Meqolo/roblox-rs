use crate::parser::parse::Parser;
use syn::{FnArg, ItemFn, Pat, Type};

pub trait Types {
    fn transform_type(&mut self, type_string: String) -> ();
}

impl Types for Parser {
    fn transform_type(&mut self, type_string: String) -> &str {
        match type_string.as_str() {
            "str" => {
                return "string";
            }
        }
    }
}
