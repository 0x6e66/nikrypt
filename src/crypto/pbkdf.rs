use crate::{crypto::hmac::hmac_sha256, prelude::W};

// https://www.rfc-editor.org/info/rfc8018/#section-5.2
pub fn pbkdf2_sha256(
    password: &[u8],
    salt: &[u8],
    iteration_count: usize,
    dk_len: usize,
) -> Vec<u8> {
    let h_len = 32;

    assert!(
        dk_len <= u32::MAX as usize * h_len,
        "pbkdf2_sha256: Desired key length is too long. Can be at most u32::MAX * 32."
    );

    assert!(
        iteration_count > 0,
        "pbkdf2_sha256: Iteration count has to be larger than one"
    );

    let l = (dk_len as f32 / h_len as f32).ceil() as usize;

    let mut t: Vec<u8> = (1..=l)
        .flat_map(|i| f(password, salt, iteration_count, i))
        .collect();

    t.truncate(dk_len);

    println!("{:02x?}", t);

    t
}

fn f(password: &[u8], salt: &[u8], iteration_count: usize, block_index: usize) -> [u8; 32] {
    let mut salt_i = vec![];
    salt_i.extend_from_slice(salt);
    salt_i.extend_from_slice(&block_index.to_be_bytes()[4..]);

    // U_1 \xor U_2 \xor ... \xor U_c
    let mut res = hmac_sha256(password, &salt_i);

    let mut last_u = res;

    for _ in 1..iteration_count {
        last_u = hmac_sha256(password, &last_u);
        res = (W(res) ^ W(last_u)).0;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let p = "password".as_bytes();
        let s = "salt".as_bytes();
        let c = 1;
        let dk_len = 20;

        let dk = pbkdf2_sha256(p, s, c, dk_len);

        let correkt_key = vec![
            0x12, 0x0f, 0xb6, 0xcf, 0xfc, 0xf8, 0xb3, 0x2c, 0x43, 0xe7, 0x22, 0x52, 0x56, 0xc4,
            0xf8, 0x37, 0xa8, 0x65, 0x48, 0xc9,
        ];

        assert_eq!(correkt_key, dk);
    }

    #[test]
    fn case2() {
        let p = "password".as_bytes();
        let s = "salt".as_bytes();
        let c = 2;
        let dk_len = 20;

        let dk = pbkdf2_sha256(p, s, c, dk_len);

        let correkt_key = vec![
            0xae, 0x4d, 0x0c, 0x95, 0xaf, 0x6b, 0x46, 0xd3, 0x2d, 0x0a, 0xdf, 0xf9, 0x28, 0xf0,
            0x6d, 0xd0, 0x2a, 0x30, 0x3f, 0x8e,
        ];

        assert_eq!(correkt_key, dk);
    }
}
