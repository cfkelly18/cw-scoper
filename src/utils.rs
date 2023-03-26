use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn check_dir(dir: &Path) -> bool {
    // todo add a list of ignored paths

    dir.to_str().unwrap().contains("/target/")
}
// todo cleanup
pub fn walk_dir(p: &PathBuf) -> Vec<PathBuf> {
    let mut scope_files: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(p) {
        let entry = entry.unwrap();

        if entry.path().extension().is_some() && entry.path().extension().unwrap() == "rs" && !check_dir(entry.path()) {
            // println!("{:#?}", entry.path());
            scope_files.push(entry.path().to_path_buf())
        };
    }

    scope_files.sort();
    scope_files
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
