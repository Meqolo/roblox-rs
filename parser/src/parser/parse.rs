use syn::visit::{self, Visit};
use syn::{Item, Macro};

use super::function::Function;
use super::r#macro::{self, Macros};

pub struct Parser {
    pub input_string: String,
    pub output_string: String,
}

pub trait Functions {
    fn parse_string(&mut self) -> ();
    fn get_items(&mut self, items: Vec<Item>) -> ();
    fn get_item_type(&mut self, item: Item) -> ();
}

impl Functions for Parser {
    fn parse_string(&mut self) {
        let syntax = syn::parse_file(&self.input_string).unwrap();

        self.get_items(syntax.items)
    }

    fn get_items(&mut self, items: Vec<Item>) {
        for item in items {
            self.get_item_type(item)
        }
    }

    fn get_item_type(&mut self, item: Item) {
        match item {
            Item::Macro(item_macro) => self.transform_item_macro(item_macro),
            Item::Fn(item_function) => self.transform_item_fn(item_function),
            _ => {
                println!("UNHANDLED ITEM: {:#?}", item);
            }
        }
    }
}
