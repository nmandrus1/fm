use std::path::PathBuf;
use std::path::Path;

use super::file::*;

/// Struct containing information about the Current working directory
#[derive(Clone, Debug)]
pub struct WorkingDir {
    cwd: PathBuf,
    files: Vec<File>,
    len: usize,
}

impl WorkingDir {
    /// Creates a new instance of WorkingDir. This can fail because it calls
    /// std::env::current_dir()
    pub fn new(dir: Option<&str>) -> anyhow::Result<Self> {
        let cwd = match dir {
            Some(dir) => PathBuf::from(dir),
            None => std::env::current_dir()?,

        };

        let mut files = Self::get_files(&cwd).unwrap();
        files.sort();
        let len = files.len();
        Ok(Self { cwd, files, len })
    }

    /// Moves the cwd to self.cwd + path
    pub fn forward(&mut self, path: PathBuf) {
        if self.len > 0 {
            self.cwd.push(path);
            self.update();
        }
    }

    /// Goes back by num directories or until it reaches the root directory
    pub fn back(&mut self, num: usize) {
        let mut i = 0;
        let mut can_go_further = true;
        while can_go_further && i < num {
            can_go_further = self.cwd.pop();
            i += 1;
        }

        self.update();
    }

    /// Function to set the cwd field in WorkingDir
    /// This is used to keep track of where you are in the file system
    pub fn set_cwd(&mut self, new_cwd: &str) -> anyhow::Result<WorkingDir> {
         WorkingDir::new(Some(new_cwd))
    }

    /// Returns the current working directory
    pub fn cwd(&self) -> &PathBuf {
        &self.cwd
    }

    pub fn files(&self) -> &[File] {
        &self.files[..]
    }
     
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Uses std::fs::read_dir() to read the contents of the directory and then uses 
    /// try_from to convert a DirEntry into a File struct
    pub fn get_files(path: &Path) -> std::io::Result<Vec<File>> {
        match std::fs::read_dir(path) {
            Ok(iter) => { 
                let files = iter.map(|d| File::from(d.unwrap())).collect();
                Ok(files)
            },
            Err(e) => Err(e),
        }
    }

    // Called after any update to WorkingDir
    fn update(&mut self) {
        self.files = Self::get_files(&self.cwd).unwrap();
        self.len = self.files.len();
    }
}

#[cfg(test)]
mod tests {
    use super::WorkingDir;
    use std::path::PathBuf;

    fn testing_working_dir() -> WorkingDir {
        WorkingDir {
            cwd: PathBuf::from("Test"),
            files: vec![],
            len: 0,
        }
    }

    // #[test]
    // fn set_test() {
    //     let mut working_dir = testing_working_dir();
        
    //     working_dir.set_cwd("other-than-Test");
    //     assert_ne!(&PathBuf::from("Test"), working_dir.cwd())
    // }
}
