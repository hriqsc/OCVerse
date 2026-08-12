use crate::schemas::post::{CreatePost, EditPost};



#[inline]
pub fn validate_post_create_post(
    metadata: &CreatePost
) ->String
{
    [ 
        validate_description(&metadata.description), 
        validate_oc_name(&metadata.oc_name), 
        validate_specie(&metadata.specie), 
        validate_sex(&metadata.sex), 
        validate_height(&metadata.height),
    ] 
    .into_iter() 
    .filter(|error| !error.is_empty()) 
    .collect::<Vec<_>>() 
    .join("\n")
}

#[inline]
pub fn validate_post_edit_post(
    metadata: &EditPost
) ->String
{
    [ 
        validate_description(&metadata.description), 
        validate_oc_name(&metadata.oc_name), 
        validate_specie(&metadata.specie), 
        validate_sex(&metadata.sex), 
        validate_height(&metadata.height),
    ] 
    .into_iter() 
    .filter(|error| !error.is_empty()) 
    .collect::<Vec<_>>() 
    .join("\n")
}



#[inline]
fn validate_description(description : &str) -> &'static str{
    if description.len() <= 1000 
    { "" }
    else {"invalid description"}
}

#[inline]
fn validate_oc_name(oc_name : &str) -> &'static str{
    if oc_name.len() <= 100 &&
    oc_name.chars().all(char::is_alphanumeric)
    { "" }
    else {"invalid oc_name"}
}

#[inline]
fn validate_specie(specie : &str) -> &'static str{
    if specie.len() <= 50 &&
    specie.chars().all(char::is_alphanumeric)
    { ""}
    else {"invalid specie"}
}

#[inline]
fn validate_sex(sex : &str) -> &'static str{
    if sex.len() == 1 &&
    (sex == "S" || sex == "M" || sex == "O")
    {""}
    else {"invalid sex"}
}


#[inline]
fn validate_height(height : &str) -> &'static str{
    if height.len() == 3 &&
    height.chars().all(char::is_numeric)
    {""}
    else {"invalid height"}
}