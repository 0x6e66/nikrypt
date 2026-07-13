use nikrypt::hash::sha2::sha_256::Hasher;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut hasher = Hasher::new();

    let path = &args[1];
    let data = std::fs::read(path).unwrap();

    hasher.hash(&data);

    let hasher = hasher.finalize();

    let digest = hasher.hex_digest();

    println!("{digest}");
}
