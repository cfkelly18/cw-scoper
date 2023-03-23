pub mod parser {

    use tree_sitter::{Language, Parser, Tree};
    use rust_code_analysis::read_file;
    use std::path::Path;
    use rust_code_analysis::{RustParser, metrics, ParserTrait, dump_node};

    pub struct ScopeParser {
        // code: Vec<u8>,
        // path: Path,
        // tree: Tree
    }

    // impl ScopeParser {
    //     pub fn new(code: Vec<u8>, path: &Path) -> RustParser {
    //         // let parser = RustParser::new(input.clone(), &path, None);

    //         return parser

            


    //     }
    // }
}