use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct PostMetadata {
    pub oc_name: String,
    pub description: String,
}