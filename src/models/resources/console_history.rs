#[derive(Default)]
pub struct ConsoleHistory {
    pub history: Vec<String>,
    pub scroll_index: usize
}