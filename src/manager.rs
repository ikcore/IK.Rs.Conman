use std::collections::VecDeque;
use super::ConmanItem;

pub struct ConmanManager {
    pub items: VecDeque<Box<dyn ConmanItem + Send>>
}
impl ConmanManager {
    pub fn new () -> ConmanManager {
        let items = VecDeque::new();
        ConmanManager { items }
    }
    pub fn add_item(&mut self, item: Box<dyn ConmanItem + Send>) {
        self.items.push_back(item);
    }
    pub fn get_next_item(&mut self) -> Option<Box<dyn ConmanItem + Send>> {
        self.items.pop_front()
    }
}