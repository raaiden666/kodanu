#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Entity {
    pub index: u32,
    pub generation: u32,
}
