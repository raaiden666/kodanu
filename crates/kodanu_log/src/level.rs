#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    #[default]
    Info,
    Trace,
    Debug,
    Warn,
    Error,
}
