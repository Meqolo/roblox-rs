extern crate quote;
extern crate syn;

mod parser;

use parser::parse::{self, Functions};

fn main() {
    let mut parser = parse::Parser {
        input_string: r#"
            println!("Hello, world!");

            fn main(string: &str, string2: &str) {
                println!("Test");
                println!("Test2");
            }
        "#
        .to_string(),
        output_string: "".to_string(),
    };

    parser.parse_string();
    println!("{}", parser.output_string);

    // println!("{}", quote!(#syntax));
}
