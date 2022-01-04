use crate::parser::parse::Parser;
use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        Block, FunctionBody, FunctionDeclaration, FunctionName, Parameter, Stmt,
    },
    tokenizer::{Token, TokenReference, TokenType},
};
use std::{borrow::Cow, cell::RefCell, rc::Rc};
use syn::{FnArg, ItemFn, Pat, Type};

pub trait Function {
    fn transform_item_fn(&mut self, item_function: ItemFn) -> ();

    fn form_lua_fn(&mut self, func_name: String, params: Vec<(String, String)>) -> ();
}

impl<'a> Function for Parser<'a> {
    fn form_lua_fn(&mut self, func_name: String, params: Vec<(String, String)>) -> () {
        let mut punctuation = Punctuated::new();
        punctuation.push(Pair::End(TokenReference::new(
            vec![],
            Token::new(TokenType::Identifier {
                identifier: Cow::from(func_name),
            }),
            vec![],
        )));

        // TODO: Parse function entries into block
        let block = Block::new();
        let mut parameters = Punctuated::new();
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
                if params.last().unwrap().0.as_str() != name.as_str() {
                    TokenReference::symbol(",").ok()
                } else {
                    None
                },
            ))
        }

        let node = FunctionDeclaration::new(FunctionName::new(punctuation))
            .with_body(FunctionBody::new().with_parameters(parameters))
            .clone();

        self.lua_ast.push((Stmt::FunctionDeclaration(node), None));
    }

    fn transform_item_fn(&mut self, item_function: ItemFn) -> () {
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

        self.form_lua_fn(name, params.clone());
    }
}
