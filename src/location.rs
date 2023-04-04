use std::fmt;

/// Enum to state between a global config/cache location
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Location {
    #[default]
    Global,
    Local,
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
