impl_fetch!(A:0);
impl_fetch!(A:0, B:1);
impl_fetch!(A:0, B:1, C:2);
impl_fetch!(A:0, B:1, C:2, D:3);
impl_fetch!(A:0, B:1, C:2, D:3, E:4);
impl_fetch!(A:0, B:1, C:2, D:3, E:4, F:5);
impl_fetch!(A:0, B:1, C:2, D:3, E:4, F:5, G:6);
impl_fetch!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7);
impl_fetch!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8);
impl_fetch!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9);
impl_fetch!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10);
impl_fetch!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11);

use crate::impl_fetch;

pub trait Fetch<'w>: Sized {
    type Item;

    fn get(&mut self, entity_index: u32) -> Option<Self::Item>;

    fn entities(&self) -> &[u32];
}
