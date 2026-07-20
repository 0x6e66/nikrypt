use crate::prelude::W;

// https://www.rfc-editor.org/info/rfc8018/#section-5.2
fn pbkdf2<const N: usize>(
    password: &[u8],
    salt: &[u8],
    iteration_count: usize,
    dk_len: usize,
    hmac_function: fn(&[u8], &[u8]) -> [u8; N],
) -> Vec<u8> {
    assert!(
        dk_len <= u32::MAX as usize * N,
        "pbkdf2_sha256: Desired key length is too long. Can be at most u32::MAX * {}.",
        N
    );

    assert!(
        iteration_count > 0,
        "pbkdf2_sha256: Iteration count has to be larger than one"
    );

    let l = (dk_len as f64 / N as f64).ceil() as usize;

    let mut t: Vec<u8> = (1..=l)
        .flat_map(|i| f(password, salt, iteration_count, i, hmac_function))
        .collect();

    t.truncate(dk_len);

    println!("{:02x?}", t);

    t
}

fn f<const N: usize>(
    password: &[u8],
    salt: &[u8],
    iteration_count: usize,
    block_index: usize,
    hmac_function: fn(&[u8], &[u8]) -> [u8; N],
) -> [u8; N] {
    let mut salt_i = vec![];
    salt_i.extend_from_slice(salt);
    salt_i.extend_from_slice(&block_index.to_be_bytes()[4..]);

    // U_1 \xor U_2 \xor ... \xor U_c
    let mut res = hmac_function(password, &salt_i);

    let mut last_u = res;

    for _ in 1..iteration_count {
        last_u = hmac_function(password, &last_u);
        res = (W(res) ^ W(last_u)).0;
    }

    res
}

macro_rules! impl_pbkdf2 {
    ($name:ident, $hmac_function:expr) => {
        pub fn $name(
            password: &[u8],
            salt: &[u8],
            iteration_count: usize,
            dk_len: usize,
        ) -> Vec<u8> {
            crate::crypto::pbkdf::pbkdf2(password, salt, iteration_count, dk_len, $hmac_function)
        }
    };
}

impl_pbkdf2!(pbkdf2_sha224, crate::crypto::hmac::hmac_sha224);
impl_pbkdf2!(pbkdf2_sha256, crate::crypto::hmac::hmac_sha256);
impl_pbkdf2!(pbkdf2_sha512, crate::crypto::hmac::hmac_sha512);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let p = "password".as_bytes();
        let s = "salt".as_bytes();
        let c = 1;
        let dk_len = 20;

        let sha256_dk = pbkdf2_sha256(p, s, c, dk_len);
        let sha256_correkt_key = vec![
            0x12, 0x0f, 0xb6, 0xcf, 0xfc, 0xf8, 0xb3, 0x2c, 0x43, 0xe7, 0x22, 0x52, 0x56, 0xc4,
            0xf8, 0x37, 0xa8, 0x65, 0x48, 0xc9,
        ];

        let sha512_dk = pbkdf2_sha512(p, s, c, dk_len);
        let sha512_correkt_key = vec![
            0x86, 0x7f, 0x70, 0xcf, 0x1a, 0xde, 0x02, 0xcf, 0xf3, 0x75, 0x25, 0x99, 0xa3, 0xa5,
            0x3d, 0xc4, 0xaf, 0x34, 0xc7, 0xa6,
        ];

        assert_eq!(sha256_correkt_key, sha256_dk);
        assert_eq!(sha512_correkt_key, sha512_dk);
    }

    #[test]
    fn case2() {
        let p = "password".as_bytes();
        let s = "salt".as_bytes();
        let c = 2;
        let dk_len = 20;

        let sha256_dk = pbkdf2_sha256(p, s, c, dk_len);
        let sha256_correkt_key = vec![
            0xae, 0x4d, 0x0c, 0x95, 0xaf, 0x6b, 0x46, 0xd3, 0x2d, 0x0a, 0xdf, 0xf9, 0x28, 0xf0,
            0x6d, 0xd0, 0x2a, 0x30, 0x3f, 0x8e,
        ];

        let sha512_dk = pbkdf2_sha512(p, s, c, dk_len);
        let sha512_correkt_key = vec![
            0xe1, 0xd9, 0xc1, 0x6a, 0xa6, 0x81, 0x70, 0x8a, 0x45, 0xf5, 0xc7, 0xc4, 0xe2, 0x15,
            0xce, 0xb6, 0x6e, 0x01, 0x1a, 0x2e,
        ];

        assert_eq!(sha256_correkt_key, sha256_dk);
        assert_eq!(sha512_correkt_key, sha512_dk);
    }

    #[test]
    fn case3() {
        let p = "passwordabc".as_bytes();
        let s = "saltabc".as_bytes();
        let c = 20_000;
        let dk_len = 40;

        let sha256_dk = pbkdf2_sha256(p, s, c, dk_len);
        let sha256_correkt_key = vec![
            0x50, 0xbd, 0x28, 0x80, 0x80, 0x3a, 0x62, 0xcd, 0x9e, 0x96, 0x6f, 0xff, 0x64, 0xc0,
            0xd4, 0x32, 0xb0, 0xed, 0xfc, 0x0d, 0xcc, 0x23, 0x17, 0x5d, 0x8e, 0xce, 0x53, 0xd3,
            0xfd, 0x04, 0x51, 0xfb, 0x84, 0x5c, 0xfa, 0xef, 0xd5, 0x73, 0xb2, 0x24,
        ];

        let sha512_dk = pbkdf2_sha512(p, s, c, dk_len);
        let sha512_correkt_key = vec![
            0x88, 0xbd, 0xa0, 0xd5, 0x29, 0x2e, 0x49, 0x10, 0xb5, 0xe8, 0x87, 0xbe, 0x60, 0xd5,
            0x3e, 0xd8, 0x51, 0xdd, 0x35, 0x2b, 0xb0, 0x0c, 0x0f, 0x99, 0xbc, 0xca, 0x67, 0xf3,
            0xa7, 0x13, 0x82, 0xef, 0x62, 0xc7, 0x40, 0x9e, 0xae, 0x7e, 0x79, 0x36,
        ];

        assert_eq!(sha256_correkt_key, sha256_dk);
        assert_eq!(sha512_correkt_key, sha512_dk);
    }
}
