mod cipher;
mod key;
mod state;
mod utils;
mod word;

use crate::{
    aes::cipher::{cipher, inv_cipher},
    aes::key::Key,
};

pub fn encrypt(key: Key, plaintext: [u8; 16]) -> Result<[u8; 16], String> {
    let nr = match key.get_size_in_bytes() {
        16 => 10,
        24 => 12,
        32 => 14,
        _ => return Err("Key has wrong length".to_owned()),
    };
    let w = key.get_round_keys().unwrap();
    Ok(cipher(plaintext, nr, w))
}

pub fn decrypt(key: Key, plaintext: [u8; 16]) -> Result<[u8; 16], String> {
    let nr = match key.get_size_in_bytes() {
        16 => 10,
        24 => 12,
        32 => 14,
        _ => return Err("Key has wrong length".to_owned()),
    };
    let w = key.get_round_keys().unwrap();
    Ok(inv_cipher(plaintext, nr, w))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt() {
        let key = Key::from([
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ]);

        let plaintext = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];

        let ciphertext = [
            0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a,
            0x0b, 0x32,
        ];

        assert_eq!(super::encrypt(key, plaintext).unwrap(), ciphertext);
    }

    #[test]
    fn decrypt() {
        let key = Key::from([
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ]);

        let ciphertext = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];

        let plaintext = [
            0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a,
            0x0b, 0x32,
        ];

        assert_eq!(super::decrypt(key, plaintext).unwrap(), ciphertext);
    }
}
