use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct PostQuery{
    pub query_type: String,
    pub query: String,
}



pub fn is_query_valid(post_query : &PostQuery)->bool{
    !post_query.query_type.is_empty() && 
    post_query.query_type.len() == 1 &&
    !post_query.query.chars().any(|c| !c.is_alphanumeric())
}