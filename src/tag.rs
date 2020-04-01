use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Tag {
    pub name: String,
}
