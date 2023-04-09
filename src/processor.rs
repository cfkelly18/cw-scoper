use syn::{parse_file, Item, ItemFn};

use std::fs::File;

use std::io::{self, BufRead};

pub fn process_functions(ast: syn::File) -> Vec<String> {
    let mut fn_names: Vec<String> = vec![];
    for item in ast.items {
        match item {
            Item::Fn(item) => fn_names.push(process_fn_data(item)),
            _ => (),
        }
    }
    fn_names
}

pub fn get_ast(file_str: String) -> syn::File {
    let ast = parse_file(&file_str).expect("unable to parse ast");

    ast
}

pub fn ast_parser(file_str: String) -> Vec<String> {
    let ast = get_ast(file_str);

    let fn_names = process_functions(ast);

    fn_names
}

fn process_fn_data(item: ItemFn) -> String {
    item.sig.ident.to_string()
}

// todo strip test lines
pub fn get_file_lines(file: &File) -> (u32, u32) {
    let mut counter: u32 = 0;
    let mut test_counter: u32 = 0;

    let lines = io::BufReader::new(file).lines();

    for line in lines {
        if line.unwrap().contains("#[test]") || test_counter >= 1 {
            // todo this will be replaced with ast methods
            test_counter += 1u32;
        }
        counter += 1u32;
    }
    return (counter, test_counter);
}
// todo -- make tests assertive and not just print
#[cfg(test)]
mod tests {
    use crate::utils::walk_dir;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    // Generate ast for all files in example directory
    #[test]
    pub fn test_generate_ast() {
        let dir = PathBuf::from("example");
        let dir_vec = walk_dir(&dir);

        for d in dir_vec {
            let code_str: String = read_to_string(&d).unwrap();

            let ast = super::get_ast(code_str);

            println!("{:#?}", ast);
        }
    }
    #[test]
    pub fn test_get_function_names() {
        let dir = PathBuf::from("example");
        let dir_vec = walk_dir(&dir);

        for d in dir_vec {
            let code_str: String = read_to_string(&d).unwrap();

            let names = super::ast_parser(code_str);

            println!("{:#?}", names);
        }
    }

    #[test]
    pub fn test_get_entry() {
        let dir = PathBuf::from("example");
        let dir_vec = walk_dir(&dir);

        for d in dir_vec {
            let code_str: String = read_to_string(&d).unwrap();

            let names = super::ast_parser(code_str);

            println!("{:#?}", names);
        }
    }
}
