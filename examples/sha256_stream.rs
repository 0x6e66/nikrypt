use nikrypt::hash::sha2::sha_256::Hasher;

fn main() {
    let mut hasher = Hasher::new();

    hasher.hash(b"hello");
    hasher.hash(b"worldasdf");

    let hasher = hasher.finalize();

    let digest = hasher.hex_digest();

    println!("{digest}");
}
