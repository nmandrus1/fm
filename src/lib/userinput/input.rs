use super::App;

pub trait Input {
    /// Returns the message asking for user input
    fn msg(&self) -> &str;

    /// Returns the input the user has provided so far
    fn input(&self) -> &str;

    /// How to handle the delete key being pressed
    fn del(&mut self, app: &mut App);

    /// How to handle a key being pressed
    fn add_to_input(&mut self, ch: char, app: &mut App);

    /// How to clear the user input 
    fn clear(&mut self);

    /// What to do on press of the Enter key
    fn on_enter(&mut self, app: &mut App);

    /// Outputs the message concatenated with the provided user input thus far
    fn output(&self) -> String {
        format!("{}{}", self.msg(), self.input())
    }
}
