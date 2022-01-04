use crate::parser::{function::Function, parse::Parser};
use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        span::ContainedSpan,
        Call, Expression, FunctionArgs, FunctionCall, Prefix, Stmt, Suffix, Value,
    },
    tokenizer::{StringLiteralQuoteType, Symbol, Token, TokenReference, TokenType},
};
use std::borrow::Cow;
use syn::ItemMacro;

fn left_paren<'a>() -> TokenReference<'a> {
    TokenReference::new(
        vec![],
        Token::new(TokenType::Symbol {
            symbol: Symbol::LeftParen,
        }),
        vec![],
    )
}

fn right_paren<'a>() -> TokenReference<'a> {
    TokenReference::new(
        vec![],
        Token::new(TokenType::Symbol {
            symbol: Symbol::RightParen,
        }),
        vec![Token::new(TokenType::Whitespace {
            characters: Cow::from("\n"),
        })],
    )
}

pub trait Macros {
    fn transform_item_macro(&mut self, item_macro: ItemMacro) -> ();
}

impl<'a> Macros for Parser<'a> {
    fn transform_item_macro(&mut self, item_macro: ItemMacro) -> () {
        let content = item_macro.mac.tokens.to_string();
        match item_macro.mac.path.segments[0].ident.to_string().as_str() {
            "println" => {
                let mut punctuation = Punctuated::new();
                punctuation.push(Pair::new(
                    Expression::Value {
                        value: Box::new(Value::String(TokenReference::new(
                            vec![],
                            Token::new(TokenType::StringLiteral {
                                literal: Cow::from(content.replace("\"", "")),
                                multi_line: None,
                                quote_type: StringLiteralQuoteType::Double,
                            }),
                            vec![],
                        ))),
                        type_assertion: None,
                    },
                    None,
                ));

                let function_call_node = FunctionCall::new(Prefix::Name(TokenReference::new(
                    vec![],
                    Token::new(TokenType::Identifier {
                        identifier: Cow::from("print"),
                    }),
                    vec![],
                )))
                .with_suffixes(vec![Suffix::Call(Call::AnonymousCall(
                    FunctionArgs::Parentheses {
                        parentheses: ContainedSpan::new(left_paren(), right_paren()),
                        arguments: punctuation,
                    },
                ))]);

                self.lua_ast
                    .push((Stmt::FunctionCall(function_call_node), None));
            }
            _ => {
                println!(
                    "Unhandled string in macros: {}",
                    item_macro.mac.path.segments[0].ident.to_string().as_str()
                )
            }
        }
    }
}
