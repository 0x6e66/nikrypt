use crate::hash::sha2::sha_256::{self, Finalized, Working};

pub fn sha224(data: &[u8]) -> [u8; 28] {
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
    pub fn digest(&self) -> [u8; 28] {
        let tmp: Vec<u8> = self.inner_hasher.state[..7]
            .iter()
            .flat_map(|a| a.to_be_bytes())
            .collect();

        let res: [u8; 28] = tmp.try_into().expect("Infallible");
        res
    }

    pub fn hex_digest(&self) -> String {
        let mut res = String::new();

        for s in self.inner_hasher.state[..7].iter() {
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
}
