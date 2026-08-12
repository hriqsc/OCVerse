use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct PostQuery{
    #[serde(rename = "type")]
    pub query_type: Option<String>,
    pub query: Option<String>,
}



// pub fn is_query_valid(post_query : &PostQuery)->bool{
//     !post_query.query_type.is_empty() && 
//     post_query.query_type.len() == 1 &&
//     !post_query.query.chars().any(|c| !c.is_alphanumeric())
// }