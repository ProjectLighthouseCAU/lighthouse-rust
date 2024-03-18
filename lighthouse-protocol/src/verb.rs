use serde::{Deserialize, Serialize};

/// A request method.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Verb {
    Post,
    Create,
    Mkdir,
    Delete,
    List,
    Get,
    Put,
    Stream,
    Stop,
    Link,
    Unlink,
    #[serde(untagged)]
    Unknown(String),
}
