// https://www.rfc-editor.org/info/rfc2104/#section-2

macro_rules! impl_hmac {
    ($sha_name:ident, $sha_hasher:ty, $digest:ident, $block_length:expr, $return: ty) => {
        pub fn $sha_name(key: &[u8], data: &[u8]) -> $return {
            let ipad = [0x36; $block_length];
            let opad = [0x5c; $block_length];

            let mut key = match key.len() {
                0..=$block_length => key.to_vec(),
                _ => {
                    let mut hasher = <$sha_hasher>::new();
                    hasher.update(key);
                    hasher.finalize().digest().to_vec()
                }
            };

            // 1
            key.resize($block_length, 0);
            let key: [u8; $block_length] = key.try_into().expect("Infallible");

            // 2
            let tmp = crate::prelude::W(key) ^ crate::prelude::W(ipad);

            // 3 + 4
            let mut h = <$sha_hasher>::new();
            h.update(&tmp.0);
            h.update(data);
            let h1 = h.finalize().digest();

            // 5
            let tmp = crate::prelude::W(key) ^ crate::prelude::W(opad);

            // 6 + 7
            let mut h = <$sha_hasher>::new();
            h.update(&tmp.0);
            h.update(&h1);

            h.finalize().$digest()
        }
    };
}

#[rustfmt::skip]
mod unformatted {
    impl_hmac!(hmac_sha224, crate::hash::sha2::sha_224::Hasher, digest, 64, [u8; 28]);
    impl_hmac!(hmac_sha256, crate::hash::sha2::sha_256::Hasher, digest, 64, [u8; 32]);
    impl_hmac!(hmac_sha512, crate::hash::sha2::sha_512::Hasher, digest, 128, [u8; 64]);

    impl_hmac!(hmac_sha224_hex, crate::hash::sha2::sha_224::Hasher, hex_digest, 64, String);
    impl_hmac!(hmac_sha256_hex, crate::hash::sha2::sha_256::Hasher, hex_digest, 64, String);
    impl_hmac!(hmac_sha512_hex, crate::hash::sha2::sha_512::Hasher, hex_digest, 128, String);
}
pub use unformatted::*;

#[cfg(test)]
mod tests {
    use super::*;

    // Test cases from https://datatracker.ietf.org/doc/html/rfc4231#section-4

    #[test]
    fn case1() {
        let key = [0xb; 20];
        let data = "Hi There".as_bytes();

        let sha224 = "896fb1128abbdf196832107cd49df33f47b4b1169912ba4f53684b22";
        let sha256 = "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7";
        let sha512 = "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cdedaa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854";

        assert_eq!(sha224, hmac_sha224_hex(&key, data));
        assert_eq!(sha256, hmac_sha256_hex(&key, data));
        assert_eq!(sha512, hmac_sha512_hex(&key, data));
    }

    #[test]
    fn case2() {
        let key = "Jefe".as_bytes();
        let data = "what do ya want for nothing?".as_bytes();

        let sha224 = "a30e01098bc6dbbf45690f3a7e9e6d0f8bbea2a39e6148008fd05e44";
        let sha256 = "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843";
        let sha512 ="164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea2505549758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737";

        assert_eq!(sha224, hmac_sha224_hex(key, data));
        assert_eq!(sha256, hmac_sha256_hex(key, data));
        assert_eq!(sha512, hmac_sha512_hex(key, data));
    }

    #[test]
    fn case3() {
        let key = [0xaa; 20];
        let data = [0xdd; 50];

        let sha224 = "7fb3cb3588c6c1f6ffa9694d7d6ad2649365b0c1f65d69d1ec8333ea";
        let sha256 = "773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe";
        let sha512 = "fa73b0089d56a284efb0f0756c890be9b1b5dbdd8ee81a3655f83e33b2279d39bf3e848279a722c806b485a47e67c807b946a337bee8942674278859e13292fb";

        assert_eq!(sha224, hmac_sha224_hex(&key, &data));
        assert_eq!(sha256, hmac_sha256_hex(&key, &data));
        assert_eq!(sha512, hmac_sha512_hex(&key, &data));
    }

    #[test]
    fn case4() {
        let key = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
        ];
        let data = [0xcd; 50];

        let sha224 = "6c11506874013cac6a2abc1bb382627cec6a90d86efc012de7afec5a";
        let sha256 = "82558a389a443c0ea4cc819899f2083a85f0faa3e578f8077a2e3ff46729665b";
        let sha512 = "b0ba465637458c6990e5a8c5f61d4af7e576d97ff94b872de76f8050361ee3dba91ca5c11aa25eb4d679275cc5788063a5f19741120c4f2de2adebeb10a298dd";

        assert_eq!(sha224, hmac_sha224_hex(&key, &data));
        assert_eq!(sha256, hmac_sha256_hex(&key, &data));
        assert_eq!(sha512, hmac_sha512_hex(&key, &data));
    }

    #[test]
    fn case5() {
        let key = [0x0c; 20];
        let data = "Test With Truncation".as_bytes();

        let sha224 = "0e2aea68a90c8d37c988bcdb9fca6fa8";
        let sha256 = "a3b6167473100ee06e0c796c2955552b";
        let sha512 = "415fad6271580a531d4179bc891d87a6";

        assert_eq!(sha224, &hmac_sha224_hex(&key, data)[..32]);
        assert_eq!(sha256, &hmac_sha256_hex(&key, data)[..32]);
        assert_eq!(sha512, &hmac_sha512_hex(&key, data)[..32]);
    }

    #[test]
    fn case6() {
        let key = [0xaa; 131];
        let data = "Test Using Larger Than Block-Size Key - Hash Key First".as_bytes();

        let sha224 = "95e9a0db962095adaebe9b2d6f0dbce2d499f112f2d2b7273fa6870e";
        let sha256 = "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54";
        let sha512 = "80b24263c7c1a3ebb71493c1dd7be8b49b46d1f41b4aeec1121b013783f8f3526b56d037e05f2598bd0fd2215d6a1e5295e64f73f63f0aec8b915a985d786598";

        assert_eq!(sha224, hmac_sha224_hex(&key, data));
        assert_eq!(sha256, hmac_sha256_hex(&key, data));
        assert_eq!(sha512, hmac_sha512_hex(&key, data));
    }

    #[test]
    fn case7() {
        let key = [0xaa; 131];
        let data = "This is a test using a larger than block-size key and a larger than block-size data. The key needs to be hashed before being used by the HMAC algorithm.".as_bytes();

        let sha224 = "3a854166ac5d9f023f54d517d0b39dbd946770db9c2b95c9f6f565d1";
        let sha256 = "9b09ffa71b942fcb27635fbcd5b0e944bfdc63644f0713938a7f51535c3a35e2";
        let sha512 = "e37b6a775dc87dbaa4dfa9f96e5e3ffddebd71f8867289865df5a32d20cdc944b6022cac3c4982b10d5eeb55c3e4de15134676fb6de0446065c97440fa8c6a58";

        assert_eq!(sha224, hmac_sha224_hex(&key, data));
        assert_eq!(sha256, hmac_sha256_hex(&key, data));
        assert_eq!(sha512, hmac_sha512_hex(&key, data));
    }
}
