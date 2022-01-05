use crate::parser::parse::Parser;
use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        span::ContainedSpan,
        Call, Expression, FunctionArgs, FunctionCall, Prefix, Stmt, Suffix, Value,
    },
    tokenizer::{StringLiteralQuoteType, Token, TokenReference, TokenType},
};
use std::borrow::Cow;
use syn::Macro;

pub trait Macros<'a> {
    fn transform_item_macro(
        &mut self,
        item_macro: Macro,
        last_item: bool,
        depth: usize,
    ) -> Option<Stmt<'a>>;
}

impl<'a> Macros<'a> for Parser<'a> {
    fn transform_item_macro(
        &mut self,
        item_macro: Macro,
        last_item: bool,
        depth: usize,
    ) -> Option<Stmt<'a>> {
        let content = item_macro.tokens.to_string();
        match item_macro.path.segments[0].ident.to_string().as_str() {
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
                    vec![Token::new(TokenType::tabs(depth))],
                    Token::new(TokenType::Identifier {
                        identifier: Cow::from("print"),
                    }),
                    vec![],
                )))
                .with_suffixes(vec![Suffix::Call(Call::AnonymousCall(
                    FunctionArgs::Parentheses {
                        parentheses: ContainedSpan::new(
                            TokenReference::symbol("(").unwrap(),
                            if last_item == true {
                                TokenReference::symbol(")").unwrap()
                            } else {
                                TokenReference::symbol(")\n").unwrap()
                            },
                        ),
                        arguments: punctuation,
                    },
                ))]);

                Some(Stmt::FunctionCall(function_call_node))
            }
            _ => {
                println!(
                    "Unhandled string in macros: {}",
                    item_macro.path.segments[0].ident.to_string().as_str()
                );
                None
            }
        }
    }
}
