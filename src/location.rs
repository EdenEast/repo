/// Enum to state between a global config/cache location
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Location {
    Global,
    Local,
}

impl Default for Location {
    fn default() -> Self {
        Location::Global
    }
}
