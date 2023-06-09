use crate::scoper::scoper::DirType;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// Checks if the directory is in the ignored list
// todo add a list of ignored paths
pub fn check_dir(dir: &Path) -> bool {
    dir.to_str().unwrap().contains("/target/")
}
/// Takes a path to a directory and returns a vector of all the .rs files in that directory and
/// all subdirectories of that directory.
pub fn walk_dir(p: &PathBuf) -> Vec<PathBuf> {
    let mut scope_files: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(p) {
        let entry = entry.unwrap();

        if entry.path().extension().is_some()
            && entry.path().extension().unwrap() == "rs"
            && !check_dir(entry.path())
        {
            // println!("{:#?}", entry.path());
            scope_files.push(entry.path().to_path_buf())
        };
    }

    scope_files.sort();
    scope_files
}
// Checks if the directory is a contract directory
pub fn is_contract_dir(dir: &PathBuf) -> bool {
    dir.to_str().unwrap().contains("/contracts/") && !is_tests_dir(dir)
}
// Checks if the directory is a package directory
pub fn is_packages_dir(dir: &PathBuf) -> bool {
    dir.to_str().unwrap().contains("/packages/") && !is_tests_dir(dir)
}
//Checks if the directory is a test directory
pub fn is_tests_dir(dir: &PathBuf) -> bool {
    dir.to_str().unwrap().contains("/test/") || dir.to_str().unwrap().contains("/tests/")
}

// Returns the type of directory
pub fn get_dir_type(dir: &PathBuf) -> DirType {
    if is_contract_dir(dir) {
        DirType::Contract
    } else if is_packages_dir(dir) {
        DirType::Package
    } else if is_tests_dir(dir) {
        DirType::Test
    } else {
        DirType::Other
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    #[test]
    pub fn test_walker() {
        let filepath: PathBuf = PathBuf::from("example/");

        super::walk_dir(&filepath);
    }
}
