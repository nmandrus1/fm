pub trait Input {
    fn msg(&self) -> &str;
    fn input(&self) -> &str;
    fn is_receiving(&self) -> bool;
    fn del(&mut self);

    fn output(&self) -> String {
        format!("{}{}", self.msg(), self.input())
    }
}

pub struct Search <'a> {
    pub msg: &'a str,
    pub input: String,
    pub is_receiving: bool,
}

impl<'a> Default for Search<'a> {
    fn default() -> Self {
        Self {
            msg: "/", 
            input: String::with_capacity(15),
            is_receiving: false
        }
    }
}

impl<'a> Input for Search<'a> {
    fn msg(&self) -> &'a str {
        &self.msg
    }

    fn input(&self) -> &str {
        &self.input
    }

    fn is_receiving(&self) -> bool {
        self.is_receiving
    }

    fn del(&mut self) {
        if self.input.len() > 1 {
            self.input.pop();
        } else {
            self.is_receiving = false
        }
    }
}
