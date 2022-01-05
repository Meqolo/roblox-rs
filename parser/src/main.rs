extern crate full_moon;
extern crate quote;
extern crate syn;

mod parser;

use parser::parse::{self, Functions};

fn main() {
    let mut parser = parse::Parser::new();

    parser.parse_string(
        r#"
        println!("Hello, world!");
        println!("Hello, world!");

        fn main(string: &str, string2: &str) -> (u32, f32) {
            println!("Test");
            println!("Test2");

            fn main2(string: &str) {
                println!("Test3");
            }
        }
    "#,
    );
    println!("{}", parser.output_string);

    // println!(
    //     "RAW DESIRED LUA: {:#?}",
    //     full_moon::parse(
    //         r#"
    //     function main(): (string, number)
    //         print("Yes")
    //     end"#
    //     )
    //     .unwrap()
    //     .nodes()
    // );
}
