#[macro_export]
macro_rules! impl_fetch {
    ($($name:ident : $index:tt),+) => {
        impl<'w, $($name),+> Fetch<'w> for ($($name,)+)
        where
            $($name: Fetch<'w>,)+
        {
            type Item = ($($name::Item,)+);

            #[inline]
            fn get(&mut self, entity_index: u32) -> Option<Self::Item> {
                Some((
                    $(self.$index.get(entity_index)?,) +
                ))
            }

            #[inline]
            fn entities(&self) -> &[u32] {
                [$(self.$index.entities(),)+].into_iter().min_by_key(|s| s.len()).unwrap()
            }
        }
    };
}
