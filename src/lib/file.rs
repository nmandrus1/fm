use super::filetype::FileType;
use super::permissions::Permissions;

use tui::style::Color;

use std::convert::From;
use std::os::unix::prelude::{MetadataExt, OsStrExt};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub ftype: FileType,
    pub perms: Permissions,
    path: PathBuf,
}

impl File {
    pub fn color(&self) -> Color {
        match self.ftype {
            FileType::Directory => Color::LightBlue,
            FileType::File => Color::White,
            FileType::Symlink => Color::Cyan,
            FileType::Executable => Color::Green,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl From<std::fs::DirEntry> for File {
    fn from(entry: std::fs::DirEntry) -> File {
        // Clippy suggestion
        // let name = String::from_utf8_lossy(entry.file_name().as_bytes().into()).to_string();
        let name = String::from_utf8(entry.file_name().as_bytes().to_vec()).unwrap();
        let path = entry.path();
        let perms = match entry.metadata() {
            Ok(mdata) => Permissions::from(mdata.mode()),
            Err(_) => Permissions::from(u32::MAX),
        };

        let ftype = match entry.file_type() {
            Ok(file_type) => {
                if file_type.is_dir() { 
                    FileType::Directory 
                } else if file_type.is_symlink() { 
                    FileType::Symlink 
                } else if perms.is_user_exec() {
                    FileType::Executable
                } else {
                    FileType::File
                }
            },
            Err(..) => FileType::File
        };

        Self { name, ftype, perms, path}
    }
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
         let name = match self.name.starts_with('.') {
            true => &self.name[1..],
            _ => &self.name,
        };
        
        let oname = match &other.name.starts_with('.') {
            true => &other.name[1..],
            _ => &other.name,
        };

        name.cmp(oname)
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        

        (&self.name, &self.ftype) == (&other.name, &other.ftype)
    }
}
impl Eq for File {}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
