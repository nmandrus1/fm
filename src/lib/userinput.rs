pub trait Input {
    fn msg(&self) -> &str;
    fn input(&self) -> &str;
    fn is_receiving(&self) -> bool;
    fn del(&mut self);
    fn append_to_inp(&mut self, ch: char);
    fn clear(&mut self);

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
            is_receiving: true
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

    fn append_to_inp(&mut self, ch: char) {
        self.input.push(ch)
    }

    fn del(&mut self) {
        if !self.input.is_empty() {
            self.input.pop();
        } else {
            self.is_receiving = false
        }
    }

    fn clear(&mut self) {
        self.input.clear();
        self.is_receiving = true;
    }
}
