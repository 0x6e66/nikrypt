use std::io::Read;

pub trait Random {
    fn random() -> Self;
}

macro_rules! impl_random {
    ($num:ty, $bytes:expr) => {
        impl Random for $num {
            fn random() -> Self {
                let mut f =
                    std::fs::File::open("/dev/urandom").expect("Could not open file /dev/urandom");
                let mut buf = [0u8; $bytes];
                f.read_exact(&mut buf)
                    .expect("Could not read from file /dev/urandom");

                <$num>::from_be_bytes(buf)
            }
        }
    };
}

impl_random!(u8, 1);
impl_random!(u16, 2);
impl_random!(u32, 4);
impl_random!(u64, 8);
impl_random!(u128, 16);
