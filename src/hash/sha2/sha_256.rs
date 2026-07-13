// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
// Section 4.2.2
const K: [u32; 64] = [
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
mod const_funcs {
    pub fn ch(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ (!x & z)
    }

    pub fn maj(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ (x & z) ^ (y & z)
    }

    pub fn sigma_big_0(x: u32) -> u32 {
        x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
    }
    pub fn sigma_big_1(x: u32) -> u32 {
        x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
    }
    pub fn sigma_small_0(x: u32) -> u32 {
        x.rotate_right(7) ^ x.rotate_right(18) ^ x >> 3
    }
    pub fn sigma_small_1(x: u32) -> u32 {
        x.rotate_right(17) ^ x.rotate_right(19) ^ x >> 10
    }
}

pub struct Working;
pub struct Finalized;

pub struct Hasher<State = Working> {
    state: [u32; 8],
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
        Self {
            #[rustfmt::skip]
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
                0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
            ],
            overflow: vec![],
            byte_count: 0,
            s: std::marker::PhantomData::<Working>,
        }
    }

    pub fn hash(&mut self, input: &[u8]) {
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
        let m = [
            u32::from_be_bytes([s[0], s[1], s[2], s[3]]),
            u32::from_be_bytes([s[4], s[5], s[6], s[7]]),
            u32::from_be_bytes([s[8], s[9], s[10], s[11]]),
            u32::from_be_bytes([s[12], s[13], s[14], s[15]]),
            u32::from_be_bytes([s[16], s[17], s[18], s[19]]),
            u32::from_be_bytes([s[20], s[21], s[22], s[23]]),
            u32::from_be_bytes([s[24], s[25], s[26], s[27]]),
            u32::from_be_bytes([s[28], s[29], s[30], s[31]]),
            u32::from_be_bytes([s[32], s[33], s[34], s[35]]),
            u32::from_be_bytes([s[36], s[37], s[38], s[39]]),
            u32::from_be_bytes([s[40], s[41], s[42], s[43]]),
            u32::from_be_bytes([s[44], s[45], s[46], s[47]]),
            u32::from_be_bytes([s[48], s[49], s[50], s[51]]),
            u32::from_be_bytes([s[52], s[53], s[54], s[55]]),
            u32::from_be_bytes([s[56], s[57], s[58], s[59]]),
            u32::from_be_bytes([s[60], s[61], s[62], s[63]]),
        ];

        // SHA-256 Hash Computation
        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
        // Section 6.2.2

        // Step 1: Prepare the message schedule
        fn w(m: &[u32; 16], t: usize) -> u32 {
            match t {
                0..=15 => m[t],
                16..=63 => const_funcs::sigma_small_1(w(m, t - 2))
                    .wrapping_add(w(m, t - 7))
                    .wrapping_add(const_funcs::sigma_small_0(w(m, t - 15)))
                    .wrapping_add(w(m, t - 16)),
                64.. => unreachable!(),
            }
        }

        // Step 2: Initialize working variables
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;

        // Step 3
        // for t in 0..64 {
        for (t, k_t) in K.iter().enumerate() {
            let t1 = h
                .wrapping_add(const_funcs::sigma_big_1(e))
                .wrapping_add(const_funcs::ch(e, f, g))
                .wrapping_add(*k_t)
                .wrapping_add(w(&m, t));
            let t2 = const_funcs::sigma_big_0(a).wrapping_add(const_funcs::maj(a, b, c));
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
