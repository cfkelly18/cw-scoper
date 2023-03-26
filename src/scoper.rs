#![allow(dead_code)]
#![allow(unused_variables)]

pub mod scoper {
    
    use std::fmt;
    use std::path::PathBuf;

    use crate::processor;
    use std::fs::File;
    
    
    use syn::{Item};

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
        lines_of_code: u32,
        //entry_points: u8,
        functions: Vec<String>,
    }

    impl FileSummary {
        pub fn new(name: &str, lines_of_code: u32, functions: Vec<String>) -> Self {
            let name = name.to_string();

            Self {
                name,
                lines_of_code,
                functions,
            }
        }
    }
    impl fmt::Display for FileSummary {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.name, self.lines_of_code)
        }
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
            // group files by dir

            // Loop the files within scope and perform processing
            for file in self.scope.clone() {
                let file_path = file.to_str();
                let code = File::open(file.clone()).expect("unable to open file"); // todo: add file checking
                let reader = File::open(file.clone()).expect("unable to open file"); // todo fix this: cant clone a file
                let content = String::new();

                let file_lines: u32 = processor::get_file_lines(code);

                let ast = syn::parse_file(&content).expect("unable to parse ast");
                let mut fn_names: Vec<String> = vec![];
                for item in ast.items {
                    match item {
                        Item::Fn(item) => fn_names.push(processor::process_fn_data(item)),
                        _ => (),
                    }
                }

                let new = FileSummary::new(file_path.unwrap(), file_lines, fn_names);
                println!("{}", new);
            }
        }
    }
}
