#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Startup,
    Begin,
    Update,
    Render,
}
