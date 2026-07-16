pub mod sha_224;
pub mod sha_256;

// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
// Section 4.1
pub(crate) mod const_functions {
    pub fn ch(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ (!x & z)
    }

    pub fn maj(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ (x & z) ^ (y & z)
    }

    // SHA-224 and SHA-256
    pub fn sigma_big_0_256(x: u32) -> u32 {
        x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
    }
    pub fn sigma_big_1_256(x: u32) -> u32 {
        x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
    }
    pub fn sigma_small_0_256(x: u32) -> u32 {
        x.rotate_right(7) ^ x.rotate_right(18) ^ x >> 3
    }
    pub fn sigma_small_1_256(x: u32) -> u32 {
        x.rotate_right(17) ^ x.rotate_right(19) ^ x >> 10
    }

    // SHA-384, SHA-512, SHA-512/224 and SHA-512/256
    pub fn sigma_big_0_512(x: u64) -> u64 {
        x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
    }
    pub fn sigma_big_1_512(x: u64) -> u64 {
        x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
    }
    pub fn sigma_small_0_512(x: u64) -> u64 {
        x.rotate_right(1) ^ x.rotate_right(8) ^ x >> 7
    }
    pub fn sigma_small_1_512(x: u64) -> u64 {
        x.rotate_right(19) ^ x.rotate_right(61) ^ x >> 6
    }
}
