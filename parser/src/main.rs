extern crate full_moon;
extern crate quote;
extern crate syn;

mod parser;

use full_moon::ast::{Ast, Block};
use parser::parse::{self, Functions};

fn main() {
    let mut parser = parse::Parser::new();

    parser.parse_string(
        r#"
        println!("Hello, world!");
        println!("Hello, world!");

        fn main(string: &str, string2: &str) {
            println!("Test");
            println!("Test2");
        }
    "#,
    );
    println!("{}", parser.output_string);

    println!(
        "RAW DESIRED LUA: {:#?}",
        full_moon::parse(
            r#"
        function main(string: string, string2: string)
        end
    "#
        )
        .unwrap()
        .nodes()
    );
}
