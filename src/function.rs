use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};
use syn::{parse_file, File, ItemFn, Meta, Token};
#[derive(Debug, Clone, PartialEq)]
pub struct CallGraph {
    pub caller: Function,
    pub callees: Option<Vec<Function>>,
}

impl CallGraph {
    pub fn new(caller: Function, callees: Option<Vec<Function>>) -> Self {
        CallGraph { caller, callees }
    }
    pub fn add_callees(&mut self, c: Vec<Function>) {
        match &mut self.callees {
            Some(callees) => callees.extend(c),
            None => self.callees = Some(c),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: String,
    is_entry_point: bool,
    is_test: bool,
}
impl Function {
    fn new(name: String, is_entry_point: bool, is_test: bool) -> Self {
        Function {
            name,
            is_entry_point,
            is_test,
        }
    }
}
pub struct FnVisitor {
    pub functions: Vec<Function>,
}

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item_fn(&mut self, i: &ItemFn) {
        let new_function = Function::new(i.sig.ident.to_string(), false, false);
        self.functions.push(new_function);
        // visit the attributes
        for a in i.attrs.iter() {
            self.visit_attribute(a)
        }
        // visiting nested functions
        visit::visit_item_fn(self, i);
    }
    fn visit_attribute(&mut self, i: &'ast syn::Attribute) {
        if i.path().is_ident("cfg_attr") {
            let nested = i
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            for meta in nested {
                match meta {
                    // #[cfg_attr(not(feature = "library"), entry_point)]
                    Meta::Path(path) if path.is_ident("entry_point") => {
                        println!("{:#?}", "FOUND ENTRY POINT");
                        self.functions.last_mut().unwrap().is_entry_point = true;
                    }

                    _ => {
                        println!("unhandled meta: {:#?}", meta);
                    }
                }
            }
        }
    }
}
// todo: actually implement the call graph

fn get_call_graph(source_code: &str) {
    let file = parse_file(source_code).unwrap();
    let mut fn_visitor = FnVisitor { functions: vec![] };
    let mut call_graph = String::new();
    let mut call_graphs: Vec<CallGraph> = vec![];

    // Find all functions in the file
    for item in &file.items {
        if let syn::Item::Fn(func) = item {
            // Get the function name
            let func_name = &func.sig.ident;
            fn_visitor.visit_item_fn(func);
        }
    }
    println!("{:#?}", fn_visitor.functions);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_basic_get_call_graph() {
        let source_code = r#"
            fn main() {
                println!("Hello, world!");
                foo();
                bar();
            }

            fn foo() {
                bar();
                println!("foo");
            }

            fn bar() {
                println!("bar");
            }
        "#;

        let _ = get_call_graph(source_code);
    }
    use crate::utils::walk_dir;
    use crate::PathBuf;
    use std::fs::read_to_string;

    // Test walk ast
    #[test]
    pub fn test_extract_entrypoints() {
        let dir = PathBuf::from("example");
        let dir_vec = walk_dir(&dir);

        let file = read_to_string(&dir_vec[1]).unwrap();

        let _ = get_call_graph(file.as_str()); // TODO: fix this
    }
}
