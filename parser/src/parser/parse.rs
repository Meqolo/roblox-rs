use syn::{Block as SynBlock, Expr, Item, Stmt as SynStmt};

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

pub trait Functions<'a> {
    fn new() -> Self;

    fn parse_string(&mut self, input: &str) -> ();
    fn get_items(&mut self, items: Vec<Item>) -> ();
    fn parse_item(&mut self, item: Item, last_item: bool, depth: usize) -> Option<Stmt<'a>>;
    fn parse_block(
        &mut self,
        block: SynBlock,
        depth: usize,
    ) -> Vec<(Stmt<'a>, Option<TokenReference<'a>>)>;
}

impl<'a> Functions<'a> for Parser<'a> {
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
        let new_items = items.to_vec();
        let items_iter = items.into_iter();

        for (index, item) in items_iter.enumerate() {
            let node_returned = self.parse_item(item, index + 1 == new_items.len(), 0);

            if let Some(node) = node_returned {
                self.lua_ast.push((node, None))
            }
        }
    }

    fn parse_item(&mut self, item: Item, last_item: bool, depth: usize) -> Option<Stmt<'a>> {
        match item {
            Item::Macro(item_macro) => self.transform_item_macro(item_macro.mac, last_item, depth),
            Item::Fn(item_function) => self.transform_item_fn(item_function, depth),
            _ => {
                println!("UNHANDLED ITEM: {:#?}", item);
                None
            }
        }
    }

    fn parse_block(
        &mut self,
        block: SynBlock,
        depth: usize,
    ) -> Vec<(Stmt<'a>, Option<TokenReference<'a>>)> {
        let mut block_vec: Vec<(Stmt<'a>, Option<TokenReference<'a>>)> = Vec::new();
        let new_stmts = block.stmts.to_vec();
        let stmts_iter = block.stmts.into_iter();

        for (index, statement_enum) in stmts_iter.enumerate() {
            let mut expr_option = None;
            let mut item_option = None;
            let mut node = None;
            match statement_enum {
                SynStmt::Semi(expr, _) => expr_option = Some(expr),
                SynStmt::Item(item) => item_option = Some(item),
                _ => println!("Unprocessed statement enum {:#?}", statement_enum),
            }

            if let Some(expr) = expr_option {
                match expr {
                    Expr::Macro(macro_expr) => {
                        node = self.transform_item_macro(
                            macro_expr.mac,
                            index + 1 == new_stmts.len(),
                            depth,
                        )
                    }
                    _ => {
                        println!("Unhandled expression in block. Node: {:#?}", expr);
                    }
                };
            } else if let Some(item) = item_option {
                match item {
                    Item::Fn(fn_item) => node = self.transform_item_fn(fn_item, depth),
                    _ => {
                        println!("Unhandled item in block. Node: {:#?}", item);
                    }
                }
            }

            if let Some(statement) = node {
                block_vec.push((statement, None))
            }
        }

        block_vec
    }
}
