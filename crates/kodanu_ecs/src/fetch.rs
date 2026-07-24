pub trait Fetch<'w>: Sized {
    type Item;

    fn get(&mut self, entity_index: u32) -> Option<Self::Item>;

    fn entities(&self) -> &[u32];
}
