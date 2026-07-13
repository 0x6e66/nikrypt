use nikrypt::hash::sha2::sha_256::Hasher;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    assert_eq!(args.len(), 2, "Supply path");

    let path = &args[1];
    let data = std::fs::read(path).unwrap();

    let mut hasher = Hasher::new();
    hasher.hash(data);

    let digest = hasher.hex_digest();

    println!("{digest}");
}
