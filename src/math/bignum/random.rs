use std::io::Read;

use crate::math::bignum::Bignum;

impl Bignum {
    pub fn generate_random(bits: usize) -> Self {
        if bits == 0 {
            panic!("Can't create Bignum with 0 bits. `bits` has to be > 0");
        }

        let num_digits = bits / 64;
        let num_bits = bits % 64;

        let mut f = std::fs::File::open("/dev/urandom").expect("Could not open file /dev/urandom");
        let mut digits = Vec::new();

        let mut buf = [0u8; 8];
        for _ in 0..num_digits {
            f.read_exact(&mut buf)
                .expect("Could not read from file /dev/urandom");
            let tmp = u64::from_be_bytes(buf);
            digits.push(tmp);
        }

        if num_bits != 0 {
            f.read_exact(&mut buf)
                .expect("Could not read from file /dev/urandom");
            let mut tmp = u64::from_be_bytes(buf);

            tmp >>= 64 - num_bits;
            digits.push(tmp);
        }

        let mut bn = Self { digits };
        bn.strip();

        bn
    }

    pub fn generate_random_prime(bits: usize) -> Self {
        let mut bn = Bignum::generate_random(bits);
        bn.set_bit(0);

        while !bn.is_prime() {
            bn = Bignum::generate_random(bits);
            bn.set_bit(0);
        }

        bn
    }
}
