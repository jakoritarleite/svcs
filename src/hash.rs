use rand::Rng;
use sha2::{Sha256, Digest};

pub fn create_hash() -> String  {
    let mut hasher = Sha256::new();

    let nonce = rand::thread_rng().gen::<u32>();

    hasher.update(nonce.to_string());

    hex::encode(hasher.finalize().as_slice())
}
