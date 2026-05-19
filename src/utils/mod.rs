use sha1::{Digest, Sha1};

pub fn sha1_hex(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);

    hex::encode(hasher.finalize())
}