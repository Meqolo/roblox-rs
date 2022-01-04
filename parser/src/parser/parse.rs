use syn::visit::{self, Visit};
use syn::{Item, Macro};

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
            // println!("{:#?}", item);
            self.get_item_type(item)
        }
    }

    fn get_item_type(&mut self, item: Item) {
        match item {
            Item::Macro(item_macro) => {
                // println!("{:#?}", item_macro);
                // r#macro::transform(item_macro);
                self.transform_item_macro(item_macro)
            }
            _ => {}
        }
    }
}
