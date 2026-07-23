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
        let a = self.0.entities();
        let b = self.1.entities();

        if a.len() <= b.len() { a } else { b }
    }
}

impl<'w, A, B, C> Fetch<'w> for (A, B, C)
where
    A: Fetch<'w>,
    B: Fetch<'w>,
    C: Fetch<'w>,
{
    type Item = (A::Item, B::Item, C::Item);

    #[inline]
    fn get(&mut self, entity_index: u32) -> Option<Self::Item> {
        Some((
            self.0.get(entity_index)?,
            self.1.get(entity_index)?,
            self.2.get(entity_index)?,
        ))
    }

    #[inline]
    fn entities(&self) -> &[u32] {
        let mut smallest = self.0.entities();

        if self.1.entities().len() < smallest.len() {
            smallest = self.1.entities();
        };

        if self.2.entities().len() < smallest.len() {
            smallest = self.2.entities();
        };

        smallest
    }
}
