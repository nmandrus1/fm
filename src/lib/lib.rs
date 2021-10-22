pub mod permissions;
pub mod workingdir;
pub mod file;
pub mod filetype;
pub mod app;
pub mod ui;
pub mod userinput;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
