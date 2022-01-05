use crate::parser::parse::Parser;

pub trait Types<'a> {
    fn transform_type(&mut self, type_string: String) -> &'a str;
}

impl<'a> Types<'a> for Parser<'a> {
    fn transform_type(&mut self, type_string: String) -> &'a str {
        match type_string.as_str() {
            "str" | "String" => "string",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64"
            | "i128" | "isize" | "f32" | "f64" => "number",
            _ => return "",
        }
    }
}
