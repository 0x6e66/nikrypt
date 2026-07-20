use crate::crypto::sha2::{sha_256, Finalized, Working};

const DIGEST_SIZE: usize = 28;
const TRUNC_SIZE: usize = DIGEST_SIZE / 4;

pub fn sha224(data: &[u8]) -> [u8; DIGEST_SIZE] {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize().digest()
}

pub fn sha224_hex(data: &[u8]) -> String {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize().hex_digest()
}

pub struct Hasher<State = Working> {
    inner_hasher: sha_256::Hasher<State>,
    s: std::marker::PhantomData<State>,
}

impl Default for Hasher<Working> {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher<Working> {
    pub fn new() -> Self {
        let inner_hasher = sha_256::Hasher::new_internal([
            0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7,
            0xbefa4fa4,
        ]);

        Self {
            inner_hasher,
            s: std::marker::PhantomData::<Working>,
        }
    }

    pub fn update(&mut self, input: &[u8]) {
        self.inner_hasher.update(input);
    }

    pub fn finalize(self) -> Hasher<Finalized> {
        let inner_hasher = self.inner_hasher.finalize();

        Hasher {
            inner_hasher,
            s: std::marker::PhantomData::<Finalized>,
        }
    }
}

impl Hasher<Finalized> {
    pub fn digest(&self) -> [u8; DIGEST_SIZE] {
        let tmp: Vec<u8> = self.inner_hasher.state[..TRUNC_SIZE]
            .iter()
            .flat_map(|a| a.to_be_bytes())
            .collect();

        let res: [u8; DIGEST_SIZE] = tmp.try_into().expect("Infallible");
        res
    }

    pub fn hex_digest(&self) -> String {
        let mut res = String::new();

        for s in self.inner_hasher.state[..TRUNC_SIZE].iter() {
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
        let hash = sha224_hex(input);
        let correct_hash = "90a3ed9e32b2aaf4c61c410eb925426119e1a9dc53d4286ade99a809";

        assert_eq!(correct_hash, hash);
    }

    #[test]
    fn case2() {
        let input = "testtest".as_bytes();
        let hash = sha224_hex(input);
        let correct_hash = "f617af1ca774ebbd6d23e8fe12c56d41d25a22d81e88f67c6c6ee0d4";

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
        let correct_hash = "080f1a95d591caa1d237328aa0de0f57b640a255240918d46afe3cd9";

        assert_eq!(correct_hash, hash);
    }

    #[test]
    fn case4() {
        let mut hasher = Hasher::new();

        let base_bytes = [
            0x33, 0xc3, 0x95, 0xc3, 0xa5, 0xc2, 0xb2, 0xc3, 0xb3, 0x01, 0x7d, 0xc2, 0x91, 0xc3,
            0xb2, 0xc4, 0x80, 0xc3, 0xa5, 0x73, 0xc2, 0x99, 0x41, 0xc2, 0xa0, 0x23, 0xc3, 0xb5,
            0x5d, 0x27, 0xc2, 0xa1, 0xd1, 0x94, 0xb2, 0x2f, 0xfd, 0xe2, 0x41, 0x08, 0x85, 0xdb,
        ];

        for _ in 0..1_000 {
            hasher.update(&base_bytes);
        }

        let hash = hasher.finalize().hex_digest();
        let correct_hash = "588f317c13783238ac67936fdc5a0843c540b81f781bb0c40d073e51";

        assert_eq!(correct_hash, hash);
    }
}
