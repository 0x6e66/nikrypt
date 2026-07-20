use crate::crypto::sha2::{sha_512, Finalized, Working};

#[derive(Debug, PartialEq)]
pub struct Truncation(usize);

impl Truncation {
    #[allow(clippy::new_ret_no_self)]
    /// specify truncation in BYTES
    pub fn new(t: usize) -> Option<Self> {
        match t {
            48 | 64.. => None,
            _ => Some(Self(t)),
        }
    }
}

pub fn sha512_t(data: &[u8], t: &Truncation) -> Vec<u8> {
    let mut hasher = Hasher::new(t);
    hasher.update(data);
    hasher.finalize().digest()
}

pub fn sha512_t_hex(data: &[u8], t: &Truncation) -> String {
    let mut hasher = Hasher::new(t);
    hasher.update(data);
    hasher.finalize().hex_digest()
}

pub struct Hasher<State = Working> {
    inner_hasher: sha_512::Hasher<State>,
    t: usize,
    s: std::marker::PhantomData<State>,
}

fn calc_init_state(t: &Truncation) -> [u64; 8] {
    let mut sha512_hasher = crate::crypto::sha2::sha_512::Hasher::new();

    let h_p = &sha512_hasher.state;
    let mut h_pp = [0u64; 8];

    for i in 0..8 {
        h_pp[i] = h_p[i] ^ 0xa5a5a5a5a5a5a5a5;
    }

    sha512_hasher.state = h_pp;
    let data = format!("SHA-512/{}", t.0 * 8);

    sha512_hasher.update(data.as_bytes());
    let sha512_hasher = sha512_hasher.finalize();

    sha512_hasher.state
}

impl Hasher<Working> {
    pub fn new(t: &Truncation) -> Self {
        let inner_hasher = sha_512::Hasher::new_internal(calc_init_state(t));

        Self {
            inner_hasher,
            t: t.0,
            s: std::marker::PhantomData::<Working>,
        }
    }

    pub fn update(&mut self, input: &[u8]) {
        self.inner_hasher.update(input);
    }

    pub fn finalize(self) -> Hasher<Finalized> {
        let t = self.t;
        let inner_hasher = self.inner_hasher.finalize();

        Hasher {
            inner_hasher,
            t,
            s: std::marker::PhantomData::<Finalized>,
        }
    }
}

impl Hasher<Finalized> {
    pub fn digest(&self) -> Vec<u8> {
        let tmp: Vec<u8> = self
            .inner_hasher
            .state
            .iter()
            .flat_map(|a| a.to_be_bytes())
            .take(self.t)
            .collect();

        tmp
    }

    pub fn hex_digest(&self) -> String {
        let mut res = String::new();

        for b in self.digest() {
            res.push_str(&format!("{b:02x}"));
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sha512_224_init_value() {
        let t = Truncation::new(28).unwrap();

        let sha_512_224_init = [
            0x8C3D37C819544DA2,
            0x73E1996689DCD4D6,
            0x1DFAB7AE32FF9C82,
            0x679DD514582F9FCF,
            0x0F6D2B697BD44DA8,
            0x77E36F7304C48942,
            0x3F9D85A86A1D36C8,
            0x1112E6AD91D692A1,
        ];

        assert_eq!(sha_512_224_init, calc_init_state(&t));
    }

    #[test]
    fn sha512_256_init_value() {
        let t = Truncation::new(32).unwrap();

        let sha_512_256_init = [
            0x22312194FC2BF72C,
            0x9F555FA3C84C64C2,
            0x2393B86B6F53B151,
            0x963877195940EABD,
            0x96283EE2A88EFFE3,
            0xBE5E1E2553863992,
            0x2B0199FC2C85B8AA,
            0x0EB72DDC81C52CA2,
        ];

        assert_eq!(sha_512_256_init, calc_init_state(&t));
    }

    #[test]
    fn wrong_truncation() {
        for t in [
            Truncation::new(48), // 384
            Truncation::new(64), // 512
            Truncation::new(65), // 520
        ] {
            assert_eq!(None, t);
        }
    }

    #[test]
    fn case1() {
        let input = "test".as_bytes();

        let t_224 = Truncation::new(28).unwrap();
        let t_256 = Truncation::new(32).unwrap();

        let hash_224 = sha512_t_hex(input, &t_224);
        let hash_256 = sha512_t_hex(input, &t_256);

        let correct_hash_224 = "06001bf08dfb17d2b54925116823be230e98b5c6c278303bc4909a8c";
        let correct_hash_256 = "3d37fe58435e0d87323dee4a2c1b339ef954de63716ee79f5747f94d974f913f";

        assert_eq!(correct_hash_224, hash_224);
        assert_eq!(correct_hash_256, hash_256);
    }

    #[test]
    fn case2() {
        let input = "testtest".as_bytes();

        let t_224 = Truncation::new(28).unwrap();
        let t_256 = Truncation::new(32).unwrap();

        let hash_224 = sha512_t_hex(input, &t_224);
        let hash_256 = sha512_t_hex(input, &t_256);

        let correct_hash_224 = "353f2beed3409bae708d05b8c33dc4b01ce1723194b215f9b0f2f40e";
        let correct_hash_256 = "14f314274868f80ab1fe84e219c7a0e30e5645593509dc67b50edd2a59d0500d";

        assert_eq!(correct_hash_224, hash_224);
        assert_eq!(correct_hash_256, hash_256);
    }

    #[test]
    fn case3() {
        let s = "opiuasdvf89pbuv4wpb98uvaw4p9buaw4vp9ubawvp49".as_bytes();

        let t_224 = Truncation::new(28).unwrap();
        let t_256 = Truncation::new(32).unwrap();

        let mut hasher_224 = Hasher::new(&t_224);
        let mut hasher_256 = Hasher::new(&t_256);

        for _ in 0..10 {
            hasher_224.update(s);
            hasher_256.update(s);
        }

        let correct_hash_224 = "d16a70eb97677c70386340b69bbea120c676716338c14db90ea281ae";
        let correct_hash_256 = "afa627eee9cb1dacde47d12577d75804dc60d15004a15ac9c44e21d40a848753";

        assert_eq!(correct_hash_224, hasher_224.finalize().hex_digest());
        assert_eq!(correct_hash_256, hasher_256.finalize().hex_digest());
    }

    #[test]
    fn case4() {
        let b = [
            0x33, 0xc3, 0x95, 0xc3, 0xa5, 0xc2, 0xb2, 0xc3, 0xb3, 0x01, 0x7d, 0xc2, 0x91, 0xc3,
            0xb2, 0xc4, 0x80, 0xc3, 0xa5, 0x73, 0xc2, 0x99, 0x41, 0xc2, 0xa0, 0x23, 0xc3, 0xb5,
            0x5d, 0x27, 0xc2, 0xa1, 0xd1, 0x94, 0xb2, 0x2f, 0xfd, 0xe2, 0x41, 0x08, 0x85, 0xdb,
        ];

        let t_224 = Truncation::new(28).unwrap();
        let t_256 = Truncation::new(32).unwrap();

        let mut hasher_224 = Hasher::new(&t_224);
        let mut hasher_256 = Hasher::new(&t_256);

        for _ in 0..1_000 {
            hasher_224.update(&b);
            hasher_256.update(&b);
        }

        let correct_hash_224 = "06fa3b9d71d32480b47ec3620ffd613df1c86d983c0a93ed5afeb2df";
        let correct_hash_256 = "6e2cd42807780ce860e8cf7fbc3fb5706bbd9f351ee025d3f73c212339418f0d";

        assert_eq!(correct_hash_224, hasher_224.finalize().hex_digest());
        assert_eq!(correct_hash_256, hasher_256.finalize().hex_digest());
    }
}
