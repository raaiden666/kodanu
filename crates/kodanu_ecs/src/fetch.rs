pub trait Fetch<'w>: Sized {
    type Item;

    fn get(&mut self, entity_index: u32) -> Option<Self::Item>;

    fn entities(&self) -> &[u32];
}

impl<'w, A, B> Fetch<'w> for (A, B)
where
    A: Fetch<'w>,
    B: Fetch<'w>,
{
    type Item = (A::Item, B::Item);

    #[inline]
    fn get(&mut self, entity_index: u32) -> Option<Self::Item> {
        Some((self.0.get(entity_index)?, self.1.get(entity_index)?))
    }

    #[inline]
    fn entities(&self) -> &[u32] {
        self.0.entities()
    }
}
