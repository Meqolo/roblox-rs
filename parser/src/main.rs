extern crate quote;
extern crate syn;

mod parser;
mod transformers;

use parser::parse::{self, Functions};
use quote::quote;
use syn::{Item, ItemMacro};
use transformers::print;

fn main() {
    // let source = ;

    let mut output = "".to_string();

    // parse::parse_string(source, &mut output);

    let mut parser = parse::Parser {
        input_string: r#"
            println!("Hello, world!");

            fn main() {
                println!("Test");
            }
        "#
        .to_string(),
        output_string: "".to_string(),
    };

    parser.parse_string();
    println!("{}", parser.output_string);

    // println!("{}", quote!(#syntax));
}
