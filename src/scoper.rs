#![allow(dead_code)]
#![allow(unused_variables)]

pub mod scoper {

    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use std::{fmt, path};

    use crate::processor;
    use std::fs::File;

    use crate::utils::get_dir_type;
    use serde::{Deserialize, Serialize};
    use syn::Item;

    pub enum ScoperMode {
        verbose,
        quick,
    }

    pub enum OutputMode {
        cli,
        json,
        txt,
    }

    // FileSummary is the struct that will be used to store the summary of each file
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct FileSummary {
        lines_of_code: u32,
        //entry_points: u8,
        functions: Vec<String>,
        path: PathBuf,
    }

    impl FileSummary {
        pub fn new(lines_of_code: u32, functions: Vec<String>, path: PathBuf) -> Self {
            Self {
                lines_of_code,
                functions, //todo fix empty vec
                path,
            }
        }
    }
    // DirType is the enum that will be used to store the type of directory
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub enum DirType {
        Contract,
        Package,
        Test,
        Other,
    }
    impl fmt::Display for DirType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                DirType::Contract => write!(f, "Contract"),
                DirType::Package => write!(f, "Package"),
                DirType::Test => write!(f, "Test"),
                DirType::Other => write!(f, "Other"),
            }
        }
    }
    // AuditDirSummary is the struct that will be used to store the summary of each directory
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct AuditDirSummary {
        directory_path: PathBuf,
        directory_files: HashMap<String, FileSummary>,
        directory_lines_of_code: u32,
        directory_type: DirType,
    }

    impl AuditDirSummary {
        pub fn new(directory_path: PathBuf, directory_type: DirType) -> Self {
            Self {
                directory_path,
                directory_files: HashMap::new(),
                directory_lines_of_code: 0,
                directory_type,
            }
        }
        pub fn get_dir_lines(&self) -> u32 {
            self.directory_lines_of_code
        }
        pub fn increment_dir_lines(&mut self, lines: u32) {
            self.directory_lines_of_code += lines;
        }
        pub fn add_file(&mut self, file: FileSummary) {
            let name = file.path.file_name().unwrap().to_str().unwrap().to_string();
            match self.directory_files.get(&name) {
                Some(_) => {
                    return;
                }
                None => {
                    self.directory_files.insert(name.clone(), file.clone());
                    self.increment_dir_lines(file.lines_of_code);
                }
            }
        }
    }
    // Summary is the struct that will be used to store the summary of the scope
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Summary {
        audit_dirs: Vec<AuditDirSummary>,
    }
    // Scoper is the main struct that will be used to process the scope
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

        pub fn process(&self) -> Summary {
            // Initialize the summary for this scope
            let audit_dirs: Vec<AuditDirSummary> = vec![];
            let mut summary: Summary = Summary { audit_dirs };

            // Loop the files within scope and perform processing
            for file in self.scope.clone() {
                let file_path = file.to_str();
                let code = File::open(file.clone()).expect("unable to open file");
                let reader = File::open(file.clone()).expect("unable to open file");
                let dir_type = get_dir_type(&file);

                // Add the directory to the summary if it doesn't exist
                if summary.audit_dirs.len() == 0 {
                    let audit_dir =
                        AuditDirSummary::new(file.parent().unwrap().to_path_buf(), dir_type);

                    summary.audit_dirs.push(audit_dir);
                } else if summary.audit_dirs.last().unwrap().directory_path
                    != file.parent().unwrap()
                {
                    let audit_dir =
                        AuditDirSummary::new(file.parent().unwrap().to_path_buf(), dir_type);

                    summary.audit_dirs.push(audit_dir);
                }

                let content = String::new();

                // Getting the number of lines in the file
                let file_lines: u32 = processor::get_file_lines(code);

                // Performing the AST parsing -- in progress
                let ast = syn::parse_file(&content).expect("unable to parse ast");
                let mut fn_names: Vec<String> = vec![];
                for item in ast.items {
                    match item {
                        Item::Fn(item) => fn_names.push(processor::process_fn_data(item)),
                        _ => (),
                    }
                }
                // Each file gets added to its respective directory within the summary
                let new = FileSummary::new(file_lines, fn_names, file);
                summary
                    .audit_dirs
                    .iter_mut()
                    .last()
                    .unwrap()
                    .add_file(new.clone());
            }
            // Debugging - remove later
            for x in summary.audit_dirs.iter() {
                println!("{}\n\n", serde_json::to_string_pretty(&x).unwrap());
            }
            return summary;
        }
    }
}
