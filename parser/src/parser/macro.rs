use crate::parser::parse::Parser;
use syn::ItemMacro;

pub trait Macros {
    fn transform_item_macro(&mut self, item_macro: ItemMacro) -> ();
}

impl Macros for Parser {
    fn transform_item_macro(&mut self, item_macro: ItemMacro) -> () {
        let content = item_macro.mac.tokens.to_string();
        match item_macro.mac.path.segments[0].ident.to_string().as_str() {
            "println" => self
                .output_string
                .push_str(format!("print({})\n", content).as_str()),
            _ => {}
        }
    }
}
