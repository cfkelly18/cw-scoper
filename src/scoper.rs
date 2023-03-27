#![allow(dead_code)]
#![allow(unused_variables)]

pub mod scoper {

    use std::fmt;
    use std::path::{Path, PathBuf};

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
    #[derive(Serialize, Deserialize, Debug)]
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
            write!(f, "(File:{}, Lines: [{}])", self.name, self.lines_of_code)
        }
    }

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

    pub struct AuditDirSummary {
        path: PathBuf,
        files: Vec<FileSummary>,
        lines_of_code: u8,
        dir_type: DirType,
    }

    impl AuditDirSummary {
        pub fn new(path: PathBuf, dir_type: DirType) -> Self {
            Self {
                path,
                files: vec![],
                lines_of_code: 0,
                dir_type,
            }
        }
    }
    impl fmt::Display for AuditDirSummary {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "(Path:{}\n, Lines: [{}]\n, Files: {}, Type: {})",
                self.path.to_str().unwrap(),
                self.lines_of_code,
                self.files
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                self.dir_type
            )
        }
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
            let auditDirs: Vec<AuditDirSummary> = vec![];
            let mut summary: Summary = Summary { auditDirs };

            // group files by dir

            // Loop the files within scope and perform processing
            for file in self.scope.clone() {
                let file_path = file.to_str();
                let code = File::open(file.clone()).expect("unable to open file");
                let reader = File::open(file.clone()).expect("unable to open file");
                let dir_type = get_dir_type(&file);
                if summary.auditDirs.len() == 0 {
                    let audit_dir =
                        AuditDirSummary::new(file.parent().unwrap().to_path_buf(), dir_type);

                    summary.auditDirs.push(audit_dir);
                    // println!("Added first audit dir: {:?}", file.parent().unwrap());
                } else if summary.auditDirs.last().unwrap().path != file.parent().unwrap() {
                    let audit_dir =
                        AuditDirSummary::new(file.parent().unwrap().to_path_buf(), dir_type);
                    summary.auditDirs.push(audit_dir);
                    // println!("Added new audit dir: {:?}", file.parent().unwrap());
                }

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
                summary.auditDirs.iter_mut().last().unwrap().files.push(new);
                // println!("{}", new);
            }
            for x in summary.auditDirs.iter() {
                println!("{}\n\n", x);
            }
        }
    }
}
