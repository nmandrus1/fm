#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Executable, 
}
