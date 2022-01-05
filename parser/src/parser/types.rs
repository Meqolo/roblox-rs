use crate::parser::parse::Parser;

pub trait Types<'a> {
    fn transform_type(&mut self, type_string: String) -> &'a str;
}

impl<'a> Types<'a> for Parser<'a> {
    fn transform_type(&mut self, type_string: String) -> &'a str {
        match type_string.as_str() {
            "str" | "String" => {
                return "string";
            }
            _ => return "",
        }
    }
}
