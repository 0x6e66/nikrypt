use crate::{
    hash::sha2::sha_256::{self, Hasher},
    prelude::*,
};

// https://www.rfc-editor.org/info/rfc2104/#section-2
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let ipad = [0x36; 64];
    let opad = [0x5c; 64];

    let mut key = match key.len() {
        0..=64 => key.to_vec(),
        65.. => sha_256::sha256(key).to_vec(),
    };

    // 1
    key.resize(64, 0);
    let key: [u8; 64] = key.try_into().expect("Infallible");

    // 2
    let tmp = W(key) ^ W(ipad);

    // 3 + 4
    let mut h = Hasher::new();
    h.hash(&tmp.0);
    h.hash(data);
    let h1 = h.finalize().digest();

    // 5
    let tmp = W(key) ^ W(opad);

    // 6 + 7
    let mut h = Hasher::new();
    h.hash(&tmp.0);
    h.hash(&h1);
    let h2 = h.finalize().digest();

    h2
}
