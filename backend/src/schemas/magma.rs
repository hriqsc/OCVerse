use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MagmaInsert {
    pub url : String,
    pub time_stamp : i64
}