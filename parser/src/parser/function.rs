use crate::parser::parse::Parser;
use syn::{FnArg, ItemFn, Pat, Type};

pub trait Function {
    fn transform_item_fn(&mut self, item_function: ItemFn) -> ();
}

impl Function for Parser {
    fn transform_item_fn(&mut self, item_function: ItemFn) -> () {
        println!("{:#?}", item_function);
        let name = item_function.sig.ident.to_string();
        let mut params: Vec<String> = Vec::new();

        for param in item_function.sig.inputs {
            match param {
                FnArg::Typed(param_info) => match *param_info.pat {
                    Pat::Ident(info) => {
                        let mut param_string = ""
                        match *param_info.ty {
                            syn::Type::Reference(fn_type) => match *fn_type.elem {
                                Type::Path(type_path) => {
                                    println!("{:#?}", type_path.path.segments[0].ident.to_string());
                                }
                                _ => {}
                            },
                            _ => println!(
                                "Unsupported Type in Function Parameter: {:#?}",
                                param_info.ty
                            ),
                        }
                        params.push(info.ident.to_string())
                    }
                    _ => println!(
                        "Unsupported Pat Type in Function Parameter: {:#?}",
                        param_info.pat
                    ),
                },
                FnArg::Receiver(_) => {
                    println!("self is not accepted as a valid function parameter")
                }
            }
        }

        self.output_string
            .push_str(format!("\nfunction {}({}) do\n", name, params.join(", ")).as_str());

        self.output_string.push_str("end\n");

        // let content = item_macro.mac.tokens.to_string();
        // match item_macro.mac.path.segments[0].ident.to_string().as_str() {
        //     "println" => self
        //         .output_string
        //         .push_str(format!("print({})", content).as_str()),
        //     _ => {}
        // }
    }
}
