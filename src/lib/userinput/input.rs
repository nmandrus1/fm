use super::App;

pub trait Input {
    fn msg(&self) -> &str;
    fn input(&self) -> &str;
    fn is_receiving(&self) -> bool;
    fn del(&mut self, app: &mut App);
    fn add_to_input(&mut self, ch: char, app: &mut App);
    fn clear(&mut self);
    fn on_enter(&mut self, app: &mut App);

    fn output(&self) -> String {
        format!("{}{}", self.msg(), self.input())
    }
}
