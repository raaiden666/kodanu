#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Startup,
    PreUpdate,
    Update,
    LateUpdate,
}
