use crate::crypto::sha2::{sha_512, Finalized, Working};

const DIGEST_SIZE: usize = 48;
const TRUNC_SIZE: usize = DIGEST_SIZE / (16 - 8);

pub fn sha384(data: &[u8]) -> [u8; DIGEST_SIZE] {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize().digest()
}

pub fn sha384_hex(data: &[u8]) -> String {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize().hex_digest()
}

pub struct Hasher<State = Working> {
    inner_hasher: sha_512::Hasher<State>,
    s: std::marker::PhantomData<State>,
}

impl Default for Hasher<Working> {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher<Working> {
    pub fn new() -> Self {
        let inner_hasher = sha_512::Hasher::new_internal([
            0xcbbb9d5dc1059ed8,
            0x629a292a367cd507,
            0x9159015a3070dd17,
            0x152fecd8f70e5939,
            0x67332667ffc00b31,
            0x8eb44a8768581511,
            0xdb0c2e0d64f98fa7,
            0x47b5481dbefa4fa4,
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

        let res: [u8; 48] = tmp.try_into().expect("Infallible");
        res
    }

    pub fn hex_digest(&self) -> String {
        let mut res = String::new();

        for s in self.inner_hasher.state[..TRUNC_SIZE].iter() {
            res.push_str(&format!("{s:016x}"));
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
        let hash = sha384_hex(input);
        let correct_hash = "768412320f7b0aa5812fce428dc4706b3cae50e02a64caa16a782249bfe8efc4b7ef1ccb126255d196047dfedf17a0a9";

        assert_eq!(correct_hash, hash);
    }

    #[test]
    fn case2() {
        let input = "testtest".as_bytes();
        let hash = sha384_hex(input);
        let correct_hash = "40e1b690e9200dd972cb29f4526a1c6597eb9bbc06bd4a2650c34dd9424cbde0327d3f3d6898d8e456f91f21fb6805c6";

        assert_eq!(correct_hash, hash);
    }

    #[test]
    fn case3() {
        let mut hasher = Hasher::new();
        let s = "opiuasdvf89pbuv4wpb98uvaw4p9buaw4vp9ubawvp49".as_bytes();

        for _ in 0..10 {
            hasher.update(s);
        }

        let hash = hasher.finalize().hex_digest();
        let correct_hash = "3a878594f665b11eee8c3d60c616ac457dfdeace1999de52b5b432e9287cb1d3445d2e18638313ff47d36c870cc079a9";

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
        let correct_hash = "8b60e1429989684d2b9284f5fb5e48d4dce3405557148faf7c528bdd58fd48e77ed8d60ff315c7c83d77b71a0685331b";

        assert_eq!(correct_hash, hash);
    }
}
