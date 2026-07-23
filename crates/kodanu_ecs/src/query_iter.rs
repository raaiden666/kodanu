use crate::Fetch;

use std::marker::PhantomData;

pub struct QueryIter<'w, I> {
    fetch: I,
    index: usize,
    marker: PhantomData<&'w ()>,
}

impl<'w, I> QueryIter<'w, I> {
    #[inline]
    pub(crate) fn new(fetch: I) -> Self {
        Self {
            fetch,
            index: 0,
            marker: PhantomData,
        }
    }
}

impl<'w, I> Iterator for QueryIter<'w, I>
where
    I: Fetch<'w>,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.fetch.entities().len() {
            let entity = self.fetch.entities()[self.index];

            self.index += 1;

            if let Some(item) = self.fetch.get(entity) {
                return Some(item);
            }
        }

        None
    }
}
