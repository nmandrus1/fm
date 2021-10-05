#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Executable, 
}
