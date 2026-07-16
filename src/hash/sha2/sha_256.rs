use crate::hash::sha2::{Finalized, Working};

// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
// Section 4.2.2
pub(crate) const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
// Section 4.1.2
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}
fn sigma_big_0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}
fn sigma_big_1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}
fn sigma_small_0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ x >> 3
}
fn sigma_small_1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ x >> 10
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize().digest()
}

pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize().hex_digest()
}

pub struct Hasher<State = Working> {
    pub(crate) state: [u32; 8],
    overflow: Vec<u8>,
    byte_count: usize,
    s: std::marker::PhantomData<State>,
}

impl Default for Hasher<Working> {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher<Working> {
    pub fn new() -> Self {
        Self::new_internal([
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
            0x5be0cd19,
        ])
    }

    pub(crate) fn new_internal(state: [u32; 8]) -> Self {
        Self {
            state,
            overflow: vec![],
            byte_count: 0,
            s: std::marker::PhantomData::<Working>,
        }
    }

    pub fn update(&mut self, input: &[u8]) {
        for s in input.chunks(64) {
            if s.len() == 64 {
                self.inner_hash_round(s);
            } else {
                // reached last block that is not a full 64 bytes (512 bits)
                self.overflow.extend_from_slice(s);
                if self.overflow.len() >= 64 {
                    let overflow_vec: Vec<u8> = self.overflow.drain(..64).collect();
                    self.inner_hash_round(&overflow_vec);
                }
                break;
            }
        }
    }

    // s has to be of length 64
    fn inner_hash_round(&mut self, s: &[u8]) {
        assert_eq!(s.len(), 64);

        // Parsing the Message
        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
        // Section 5.2.1
        let mut w = [0u32; 64];
        for i in 0..16 {
            let b = 4 * i;
            w[i] = u32::from_be_bytes([s[b], s[b + 1], s[b + 2], s[b + 3]]);
        }

        // SHA-256 Hash Computation
        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
        // Section 6.2.2

        // Step 1: Prepare the message schedule
        for i in 16..64 {
            w[i] = sigma_small_1(w[i - 2])
                .wrapping_add(w[i - 7])
                .wrapping_add(sigma_small_0(w[i - 15]))
                .wrapping_add(w[i - 16]);
        }

        // Step 2: Initialize working variables
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;

        // Step 3
        for (t, k_t) in K.iter().enumerate() {
            let t1 = h
                .wrapping_add(sigma_big_1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(*k_t)
                .wrapping_add(w[t]);
            let t2 = sigma_big_0(a).wrapping_add(maj(a, b, c));
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        // Step 4
        self.state = [
            a.wrapping_add(self.state[0]),
            b.wrapping_add(self.state[1]),
            c.wrapping_add(self.state[2]),
            d.wrapping_add(self.state[3]),
            e.wrapping_add(self.state[4]),
            f.wrapping_add(self.state[5]),
            g.wrapping_add(self.state[6]),
            h.wrapping_add(self.state[7]),
        ];

        self.byte_count += 64;
    }

    pub fn finalize(mut self) -> Hasher<Finalized> {
        // Padding
        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
        // Section 5.1.1
        let total_length_bits = (self.byte_count + self.overflow.len()) * 8;

        self.overflow.push(0x80);
        while self.overflow.len() % 64 != 56 {
            self.overflow.push(0);
        }

        for b in total_length_bits.to_be_bytes().to_vec() {
            self.overflow.push(b);
        }

        assert_eq!(self.overflow.len() % 64, 0);

        while !self.overflow.is_empty() {
            let s: Vec<u8> = self.overflow.drain(0..64).collect();
            self.inner_hash_round(&s);
        }

        Hasher {
            state: self.state,
            overflow: self.overflow,
            byte_count: self.byte_count,
            s: std::marker::PhantomData::<Finalized>,
        }
    }
}

impl Hasher<Finalized> {
    pub fn digest(&self) -> [u8; 32] {
        let tmp: Vec<u8> = self
            .state
            .into_iter()
            .flat_map(|a| a.to_be_bytes())
            .collect();

        let res: [u8; 32] = tmp.try_into().expect("Infallible");
        res
    }

    pub fn hex_digest(&self) -> String {
        let mut res = String::new();

        for s in self.state {
            res.push_str(&format!("{s:08x}"));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "test".as_bytes();
        let hash = sha256_hex(input);
        let correct_hash = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08";

        assert_eq!(correct_hash, hash);
    }

    #[test]
    fn case2() {
        let input = "testtest".as_bytes();
        let hash = sha256_hex(input);
        let correct_hash = "37268335dd6931045bdcdf92623ff819a64244b53d0e746d438797349d4da578";

        assert_eq!(correct_hash, hash);
    }

    #[test]
    fn case3() {
        let mut hasher = Hasher::new();
        for s in [
            "very very very very very very very very very very very very ",
            "very very very very very very very very very very very very ",
            "very very very very very very very very long test",
        ] {
            hasher.update(s.as_bytes());
        }

        let hash = hasher.finalize().hex_digest();
        let correct_hash = "0267cd70ce42810aff67379951a9111d735c40f63eede5683413ba93b9086021";

        assert_eq!(correct_hash, hash);
    }
}
