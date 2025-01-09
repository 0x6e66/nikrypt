#[derive(Debug, Clone)]
pub struct UBignum<const NUM_DIGITS: usize> {
    pub(crate) digits: [u64; NUM_DIGITS],
    pub(crate) pos: usize,
}

impl<const NUM_DIGITS: usize> UBignum<NUM_DIGITS> {
    pub fn new() -> Self {
        Self::zero()
    }

    pub fn zero() -> Self {
        Self {
            digits: [0u64; NUM_DIGITS],
            pos: 0,
        }
    }

    pub fn one() -> Self {
        let mut digits = [0u64; NUM_DIGITS];
        digits[0] = 1;
        Self { digits, pos: 0 }
    }

    pub fn is_zero(&self) -> bool {
        self.pos == 0 && self.digits[0] == 0
    }

    pub fn is_empty(&self) -> bool {
        self.is_zero()
    }

    pub fn is_even(&self) -> bool {
        self.digits[0] % 2 == 0
    }

    pub fn len(&self) -> usize {
        self.pos + 1
    }

    fn calc_pos(length: usize) -> usize {
        if length <= 2 {
            0
        } else if length % 16 == 0 {
            length / 16 - 1
        } else {
            length / 16
        }
    }

    pub fn try_from_hex_string(s: &str) -> Result<Self, std::num::ParseIntError> {
        let s = s.trim_start_matches("0x");
        let s = s.trim_start_matches('0');

        let mut bignum = Self::new();
        let len = s.len();

        bignum.pos = Self::calc_pos(len);

        for i in 0..len / 16 {
            let b = &s[len - (16 * i + 16)..len - 16 * i];
            let b = u64::from_str_radix(b, 16)?;
            bignum.digits[i] = b;
        }

        if len % 16 != 0 {
            let b = &s[0..len % 16];
            let b = u64::from_str_radix(b, 16)?;
            bignum.digits[len / 16] = b;
        }

        Ok(bignum)
    }

    pub fn to_hex_string(&self) -> String {
        if self.pos == 0 && self.digits[0] == 0 {
            return String::from("0x0");
        }

        let mut res = String::new();
        let mut leading_zeros = true;

        for b in self.digits.iter().rev() {
            if *b == 0 && leading_zeros {
                continue;
            } else if *b != 0 {
                leading_zeros = false;
            }

            res.push_str(&format!("{:016x}", b));
        }

        let res = res.trim_start_matches('0');

        format!("0x{}", res)
    }
}

impl<const N: usize> PartialEq for UBignum<N> {
    fn eq(&self, other: &Self) -> bool {
        if self.pos != other.pos {
            return false;
        }

        for (a, b) in self
            .digits
            .iter()
            .rev()
            .take(self.len())
            .zip(other.digits.iter().rev().take(other.len()))
        {
            if a != b {
                return false;
            }
        }

        true
    }
}

impl<const N: usize> From<usize> for UBignum<N> {
    fn from(value: usize) -> Self {
        let mut digits = [0u64; N];
        digits[0] = value as u64;
        Self { digits, pos: 0 }
    }
}

impl<const NUM_DIGITS: usize> Default for UBignum<NUM_DIGITS> {
    fn default() -> Self {
        Self::zero()
    }
}
