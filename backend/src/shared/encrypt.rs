use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Sha256, Digest};

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    URL_SAFE_NO_PAD.encode(hasher.finalize())
}