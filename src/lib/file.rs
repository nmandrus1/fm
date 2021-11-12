use super::filetype::FileType;
use super::permissions::Permissions;

use tui::style::Color;

use std::convert::From;
use std::os::unix::prelude::{MetadataExt, OsStrExt};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub ftype: FileType,
    pub perms: Permissions,
    pub path: PathBuf,
    pub size: u64,
    pub is_selected: bool,
}

impl File {
    /// Gets a color based on the FileType of the File
    pub fn color(&self) -> Color {
        if self.is_selected {
            return Color::Rgb(213, 0, 255)
        }

        match self.ftype {
            FileType::Directory => Color::LightBlue,
            FileType::File => Color::White,
            FileType::Symlink => Color::Cyan,
            FileType::Executable => Color::Green,
        }
    }

    /// Returns a Refernce to the path of the current File
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Converts size in raw bytes to a human readable format
    pub fn size_to_readable(&self) -> String {
        let size = self.size;
        if size < 1000 {
            return format!("{} B", size)
        } else {
            let mut iter = IntoIterator::into_iter(['k', 'M', 'G', 'T', 'E'])
                .enumerate()
                .skip_while(|(i, _)| size / ((*i + 1) * 1000) as u64 > 999_950);
            let (i, ch) = iter.next().unwrap();
            return format!("{:.2} {}B", size / ((i + 1) * 1000) as u64, ch)
        }
    }

    /// updates the size field of the file
    pub fn update_size(&mut self) {
        self.size = std::fs::metadata(self.path()).unwrap().size();
    }

    pub fn update(&mut self) {
        self.name = String::from_utf8(self.path().file_name().unwrap().as_bytes().to_vec()).unwrap()
    }
}

impl From<std::path::PathBuf> for File {
    fn from(path: std::path::PathBuf) -> Self {
        let name = String::from_utf8(path.file_name().unwrap().as_bytes().to_vec()).unwrap();
        let mdata = path.metadata();
        let (perms, size, ftype) = match mdata {
            Ok(mdata) => {
                (Permissions::from(mdata.mode()), mdata.size(), Some(mdata.file_type()))
            }
            Err(_) => (Permissions::from(u32::MAX), 0, None)
        };

        let ftype = match ftype {
            Some(f) => {
                if f.is_dir() {
                    FileType::Directory
                } else if f.is_symlink() {
                    FileType::Symlink
                } else {
                    FileType::File
                }
            },
            
            None => FileType::File
        };

        let is_selected = false;

        Self { name, ftype, perms, path, size, is_selected }
    }
}

impl From<std::fs::DirEntry> for File {
    fn from(entry: std::fs::DirEntry) -> File {
        let name = String::from_utf8(entry.file_name().as_bytes().to_vec()).unwrap();
        let path = entry.path();
        let (perms, size) = match entry.metadata() {
            Ok(mdata) => {
                (Permissions::from(mdata.mode()), mdata.size())
            }
            Err(_) => (Permissions::from(u32::MAX), 0)
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

        let is_selected = false;

        Self { name, ftype, perms, path, size, is_selected }
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
