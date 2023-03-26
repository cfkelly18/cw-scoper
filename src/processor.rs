use syn::{
    parse::{self, Parse},
    parse_quote, Attribute, Expr, Ident, ItemFn, MetaList, Token,
};

use std::fs::File;
use std::io::Lines;
use std::io::{self, BufRead, BufReader};

//for now just returns function names
//todo process attributes to detect entrypoints
pub fn process_fn_data(function: ItemFn) -> String {
    let function_name = function.sig.ident.to_string();
    // let attribute: Vec<Attribute> = function.attrs;
    // // let entry_point = attribute.iter().map(|at: &Attribute| parse_quote!(at)).collect::<Vec<_>>();
    // for at in attribute{
    //     println!("{:#?}", "___________");
    //     at.meta.require_name_value();
    //     at.parse_args_with(Attribute::parse_outer).unwrap();

    return function_name;
    // }
}

// todo strip test lines
pub fn get_file_lines(file: File) -> u32 {
    let mut counter: u32 = 0;
    let lines = io::BufReader::new(file).lines();

    for _ in lines {
        counter += 1u32;
    }
    return counter;
}
