use syn::Item;

use super::function::Function;
use super::r#macro::Macros;
use full_moon::{
    ast::{Ast, Block, Stmt},
    tokenizer::{Token, TokenReference, TokenType},
};

pub struct Parser<'a> {
    pub output_string: String,
    pub lua_ast: Vec<(Stmt<'a>, Option<TokenReference<'a>>)>,
}

pub trait Functions {
    fn new() -> Self;

    fn parse_string(&mut self, input: &str) -> ();
    fn get_items(&mut self, items: Vec<Item>) -> ();
    fn get_item_type(&mut self, item: Item) -> ();

    // fn append_to_lua_ast(&mut self, new_node: Stmt) -> ();
}

impl<'a> Functions for Parser<'a> {
    fn new() -> Self {
        Self {
            output_string: String::new(),
            lua_ast: vec![],
        }
    }

    fn parse_string(&mut self, input: &str) {
        let syntax = syn::parse_file(&input.to_string()).unwrap();
        self.get_items(syntax.items);

        let mut block = Block::new();
        block = block.with_stmts(self.lua_ast.to_vec());

        let mut ast = Ast::from_tokens(vec![Token::new(TokenType::Eof)]).unwrap();
        ast = ast.with_nodes(block);

        self.output_string = full_moon::print(&ast);
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
