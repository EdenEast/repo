use std::fmt;

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

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ser = match self {
            Location::Global => "Global",
            Location::Local => "Local",
        };

        write!(f, "{}", ser)
    }
}
