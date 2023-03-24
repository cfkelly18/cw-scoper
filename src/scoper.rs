#![allow(dead_code)]
#![allow(unused_variables)]

pub mod scoper {
    use crate::parser;
    use std::path::PathBuf;

    use std::fs::File;
    use std::io::Read;
    use syn::parse_file;

    pub enum ScoperMode {
        verbose,
        quick,
    }

    pub enum OutputMode {
        cli,
        json,
        txt,
    }
    pub struct FileSummary {
        name: String,
        lines_of_code: u8,
        entry_points: u8,
    }

    pub struct AuditDirSummary {
        name: String,
        files: Vec<FileSummary>,
        lines_of_code: u8,
    }

    pub struct Summary {
        auditDirs: Vec<AuditDirSummary>,
    }

    pub struct ScoperResponse {
        summary: Summary,
    }

    pub struct Scoper {
        scope: Vec<PathBuf>,
        mode: ScoperMode,
        output: OutputMode,
    }

    impl Scoper {
        pub fn new(scope: Vec<PathBuf>, mode: ScoperMode, output: OutputMode) -> Self {
            Self {
                scope,
                mode,
                output,
            }
        }

        pub fn process(&self) {
            for file in self.scope.clone() {
                let file_path = file.to_str();
                let mut code = File::open(file).expect("unable to open file"); // todo: add file checking
                let mut content = String::new();

                code.read_to_string(&mut content);
                let ast = syn::parse_file(&content).expect("unable to parse ast");

                println!("{:#?} items", ast);
            }
        }
    }
}
