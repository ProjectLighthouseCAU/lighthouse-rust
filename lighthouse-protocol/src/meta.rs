use serde::{Deserialize, Serialize};

/// Additional flags set by the client.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub struct Meta {
    /// Setting this flag on a LIST request specifies that only the first level
    /// should be listed (instead of the full tree).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonrecursive: Option<bool>,
}
