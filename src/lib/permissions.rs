use std::convert::From;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Permissions {
    perms: u32
}

impl Permissions {
    pub fn is_user_exec(&self) -> bool {
        self.perms & 1 == 1
    }

    pub fn is_user_write(&self) -> bool {
        (self.perms >> 1) & 1 == 1
    } 

    pub fn is_user_read(&self) -> bool {
        (self.perms >> 2) & 1 == 1
    } 

    pub fn is_valid(&self) -> bool {
        self.perms < 512
    }
}

impl From<u32> for Permissions {
    fn from(num: u32) -> Self {
        Self { perms: num & 511 }
     }
}

impl std::fmt::Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        if !self.is_valid() { 
            write!(f, "Error Reading Permissions!")?;
            return Ok(())
        }

        let mut arr: [u8; 9] = [0; 9];
        arr
            .iter_mut()
            .enumerate()
            .map(|(i, _)| (i, (self.perms >> i) & 0b1))
            .rev()
            .for_each(|(i, b)| {
                let rwx = i % 3;
                if rwx == 0 {
                    match b {
                        0 => write!(f, "-").unwrap(),
                        1 => write!(f, "x").unwrap(),
                        _ => write!(f, "?").unwrap(),
                    }
                } else if rwx == 1 {
                    match b {
                        0 => write!(f, "-").unwrap(),
                        1 => write!(f, "w").unwrap(),
                        _ => write!(f, "?").unwrap(),
                    } 
                } else {
                    match b {
                        0 => write!(f, "-").unwrap(),
                        1 => write!(f, "r").unwrap(),
                        _ => write!(f, "?").unwrap(),
                    } 
                }
            });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Permissions;

    fn testing_perm_all() -> Permissions {
        Permissions::from(511)
    }

    fn testing_perm_none() -> Permissions {
        Permissions::from(0)
    }

    #[test]
    fn test_is_user_exec() {
        let perm = testing_perm_all();
        assert!(perm.is_user_exec())
    }

    #[test]
    fn test_user_not_exec() {
        let perm = testing_perm_none();
        // passes if user does not have exec perms
        assert!(!perm.is_user_exec());
    }

     #[test]
    fn test_is_user_write() {
        let perm = testing_perm_all();
        assert!(perm.is_user_write())
    }

    #[test]
    fn test_user_not_write() {
        let perm = testing_perm_none();
        // passes if user does not have exec perms
        assert!(!perm.is_user_write());
    }

    #[test]
    fn test_is_user_read() {
        let perm = testing_perm_all();
        assert!(perm.is_user_read())
    }

    #[test]
    fn test_user_not_read() {
        let perm = testing_perm_none();
        // passes if user does not have exec perms
        assert!(!perm.is_user_read());
    } 

    #[test]
    fn test_display1() {
        let perm = testing_perm_all();
        assert_eq!(String::from("rwxrwxrwx"), format!("{}", perm))
    }

    #[test]
    fn test_display2() {
        let perm = testing_perm_none();
        assert_eq!(String::from("---------"), format!("{}", perm))
    }
}
