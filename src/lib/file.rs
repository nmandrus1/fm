use super::filetype::FileType;

use tui::style::Color;

use std::convert::From;
use std::os::unix::prelude::{MetadataExt, OsStrExt};

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub ftype: FileType
}

impl File {
    pub fn color(&self) -> Color {
        match self.ftype {
            FileType::Directory => Color::Blue,
            FileType::File => Color::White,
            FileType::Symlink => Color::Cyan,
            FileType::Executable => Color::Green,
        }
    }
}

impl From<std::fs::DirEntry> for File {
    fn from(entry: std::fs::DirEntry) -> File {
        let name = String::from_utf8_lossy(entry.file_name().as_bytes().into()).to_string();
        let ftype = match entry.file_type() {
            Ok(file_type) => {
                if file_type.is_dir() { 
                    FileType::Directory 
                } else if file_type.is_symlink() { 
                    FileType::Symlink 
                } else {
                    match entry.metadata() {
                        Ok(mdata) => {
                            let perms = mdata.mode();
                            if perms & 0b1 == 1 {
                                FileType::Executable
                            } else  {
                                FileType::File
                            }
                        },
                        Err(..) => FileType::File
                    }
                }
            },
            Err(..) => FileType::File
        };

        Self { name, ftype }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
