pub type Value = i32;

pub(super) trait TreeElement {
    fn contains(&self, value: Value) -> bool;
    fn insert(&mut self, value: Value);
    fn size(&self) -> usize;
}
