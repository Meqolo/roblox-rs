use crate::parser::{parse::Parser, types::Types};
use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        span::ContainedSpan,
        types::{TypeInfo, TypeSpecifier},
        Block, FunctionBody, FunctionDeclaration, FunctionName, Parameter, Stmt,
    },
    tokenizer::{Symbol, Token, TokenReference, TokenType},
};
use std::{borrow::Cow, vec};
use syn::{FnArg, ItemFn, Pat, Type};

use super::parse::Functions;

pub trait Function<'a> {
    fn form_lua_fn(
        &mut self,
        func_name: String,
        params: Vec<(String, String)>,
        child_stmts: Vec<(Stmt<'a>, Option<TokenReference<'a>>)>,
        depth: usize,
    ) -> Stmt<'a>;

    fn transform_item_fn(&mut self, item_function: ItemFn, depth: usize) -> Option<Stmt<'a>>;
}

impl<'a> Function<'a> for Parser<'a> {
    fn form_lua_fn(
        &mut self,
        func_name: String,
        params: Vec<(String, String)>,
        child_stmts: Vec<(Stmt<'a>, Option<TokenReference<'a>>)>,
        depth: usize,
    ) -> Stmt<'a> {
        let mut punctuation = Punctuated::new();
        punctuation.push(Pair::End(TokenReference::new(
            vec![],
            Token::new(TokenType::Identifier {
                identifier: Cow::from(func_name),
            }),
            vec![],
        )));

        let block = Block::new().with_stmts(child_stmts);
        let mut parameters = Punctuated::new();
        let mut type_specifiers = Vec::new();
        let new_params = params.to_vec();
        let params_iter = params.into_iter();

        for (index, (name, type_name)) in params_iter.enumerate() {
            parameters.push(Pair::new(
                Parameter::Name(TokenReference::new(
                    vec![],
                    Token::new(TokenType::Identifier {
                        identifier: Cow::from(name),
                    }),
                    vec![],
                )),
                if index + 1 != new_params.len() {
                    TokenReference::symbol(", ").ok()
                } else {
                    None
                },
            ));

            type_specifiers.push(Some(TypeSpecifier::new(TypeInfo::Basic(
                TokenReference::new(
                    vec![],
                    Token::new(TokenType::Identifier {
                        identifier: Cow::from(self.transform_type(type_name)),
                    }),
                    vec![],
                ),
            ))))
        }

        let node = FunctionDeclaration::new(FunctionName::new(punctuation))
            .with_body(
                FunctionBody::new()
                    .with_parameters(parameters)
                    .with_type_specifiers(type_specifiers)
                    .with_parameters_parentheses(ContainedSpan::new(
                        TokenReference::symbol("(").unwrap(),
                        TokenReference::symbol(")\n").unwrap(),
                    ))
                    .with_block(block)
                    .with_end_token(TokenReference::new(
                        vec![
                            Token::new(TokenType::Whitespace {
                                characters: Cow::from("\n"),
                            }),
                            Token::new(TokenType::tabs(depth)),
                        ],
                        Token::new(TokenType::Symbol {
                            symbol: Symbol::End,
                        }),
                        vec![],
                    )),
            )
            .with_function_token(TokenReference::new(
                vec![
                    Token::new(TokenType::Whitespace {
                        characters: Cow::from("\n"),
                    }),
                    Token::new(TokenType::tabs(depth)),
                ],
                Token::new(TokenType::Symbol {
                    symbol: Symbol::Function,
                }),
                vec![Token::new(TokenType::spaces(1))],
            ))
            .clone();

        Stmt::FunctionDeclaration(node)
    }

    fn transform_item_fn(&mut self, item_function: ItemFn, depth: usize) -> Option<Stmt<'a>> {
        let name = item_function.sig.ident.to_string();
        let mut params: Vec<(String, String)> = Vec::new();

        for parameter in item_function.sig.inputs {
            if let FnArg::Typed(pat) = parameter {
                if let Pat::Ident(parameter_name) = &*pat.pat {
                    let name = parameter_name.ident.to_string();

                    if let Type::Reference(ty) = &*pat.ty {
                        if let Type::Path(elem) = &*ty.elem {
                            let type_name = elem.path.segments.first().unwrap().ident.to_string();

                            params.push((name, type_name))
                        }
                    }
                }
            }
        }

        let block_vec = self.parse_block(*item_function.block, depth + 1);
        Some(self.form_lua_fn(name, params.clone(), block_vec, depth))
    }
}
