

pub mod scoper {
    use std::path::{Path, PathBuf};
    use crate::parser;
    use rust_code_analysis::{RustParser, metrics, ParserTrait, dump_node, Metrics, dump_root};
    use rust_code_analysis::read_file;
    pub enum ScoperMode {
        verbose,
        quick
    }

    pub enum OutputMode {
        cli,
        json,
        txt
    }
    pub struct FileSummary {
        name: String,
        lines_of_code: u8,
        entry_points: u8,


    }

    pub struct AuditDirSummary {
        name: String,
        files: Vec<FileSummary>,
        lines_of_code: u8
    }

    pub struct Summary {
        auditDirs: Vec<AuditDirSummary>
    }

    pub struct ScoperResponse<> {
        summary: Summary
    }

    pub struct Scoper {
        scope: Vec<PathBuf>,
        mode: ScoperMode,
        output: OutputMode 
    }

    impl Scoper {
        pub fn new(scope: Vec<PathBuf>, mode: ScoperMode, output: OutputMode) -> Self {
           
            Self {
                scope,
                mode,
                output
            }
        }
        
        pub fn process(&self) {
            
            for file in self.scope.clone() {
                //et parser = parser::parusrser::ScopeParser::new(self.code, path);
                let code = read_file(&file).unwrap();
                let parser = RustParser::new(code.clone(), &file, None);
                
                let space = metrics(&parser, &file).unwrap();
                let file_metrics = dump_root(&space).unwrap();

                print!("{:?}", file_metrics);
            }
        }
    }
}