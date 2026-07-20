mod addition;
mod bits;
mod comparison;
mod division;
mod euclid;
mod multiplication;
mod pow;
mod prime;
mod random;
mod subtraction;

#[derive(Debug, Clone)]
pub struct Bignum {
    digits: Vec<u64>,
    sign: bool,
}

impl Bignum {
    pub fn new() -> Self {
        Self {
            digits: vec![0],
            sign: false,
        }
    }

    pub fn len(&self) -> usize {
        self.digits.len()
    }

    pub fn len_bits(&self) -> usize {
        let mut bits = self.len() * 64;
        let mut tmp = *self.digits.last().unwrap();

        let test = 1 << 63;
        while (tmp & test) == 0 {
            tmp <<= 1;
            bits -= 1;
        }

        bits
    }

    pub fn set_value(&mut self, value: u64) {
        self.digits = vec![value];
        self.sign = false;
    }

    pub fn is_zero(&self) -> bool {
        self.digits.len() == 1 && self.digits[0] == 0
    }

    pub fn set_zero(&mut self) {
        self.digits = vec![0];
        self.sign = false;
    }

    pub fn zero() -> Self {
        Self {
            digits: vec![0],
            sign: false,
        }
    }

    pub fn is_one(&self) -> bool {
        self.digits.len() == 1 && self.digits[0] == 1 && !self.sign
    }

    pub fn set_one(&mut self) {
        self.digits = vec![1];
        self.sign = false;
    }

    pub fn one() -> Self {
        Self {
            digits: vec![1],
            sign: false,
        }
    }

    pub fn toggle_sign(&mut self) {
        if !self.is_zero() {
            self.sign = !self.sign
        }
    }

    pub fn unset_sign(&mut self) {
        self.sign = false
    }

    pub fn set_sign(&mut self) {
        self.sign = true
    }

    pub fn is_signed(&self) -> bool {
        self.sign
    }

    pub fn is_empty(&self) -> bool {
        self.is_zero()
    }

    pub fn is_even(&self) -> bool {
        self.digits[0].is_multiple_of(2)
    }

    pub fn to_hex_string(&self) -> String {
        if self.digits.len() == 1 && self.digits[0] == 0 {
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

        let res = res.trim_start_matches('0').to_string();

        if self.sign {
            format!("-0x{}", res)
        } else {
            format!("0x{}", res)
        }
    }

    pub fn try_from_hex_string(t: &str) -> Result<Self, std::num::ParseIntError> {
        let mut sign = t.starts_with("-0x");

        let s = t.trim_start_matches("-");
        let s = s.trim_start_matches("0x");
        let s = s.trim_start_matches('0');

        let mut vec = vec![];

        let len = s.len();
        for i in 0..len / 16 {
            let b = &s[len - (16 * i + 16)..len - 16 * i];
            let b = u64::from_str_radix(b, 16)?;
            vec.push(b);
        }

        if !len.is_multiple_of(16) {
            let b = &s[0..len % 16];
            let b = u64::from_str_radix(b, 16)?;
            vec.push(b);
        }

        if vec == [0] || vec.is_empty() {
            sign = false;
        }

        let mut b = Self { digits: vec, sign };
        b.strip();

        Ok(b)
    }

    fn strip(&mut self) {
        let mut count = 0;

        for b in self.digits.iter().rev() {
            if *b != 0 {
                break;
            }
            count += 1;
        }

        self.digits.resize(self.digits.len() - count, 0u64);

        if self.digits.is_empty() {
            self.set_zero();
        }
    }
}

impl std::ops::Neg for Bignum {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.toggle_sign();
        self
    }
}

impl Default for Bignum {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Bignum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

impl From<u128> for Bignum {
    fn from(value: u128) -> Self {
        let mut res = Self {
            digits: vec![value as u64, (value >> 64) as u64],
            sign: false,
        };
        res.strip();
        res
    }
}

impl From<i128> for Bignum {
    fn from(value: i128) -> Self {
        let sign = value < 0;
        let value = value as u128;

        let mut bn = Bignum::from(value);
        bn.sign = sign;

        bn
    }
}

impl From<i32> for Bignum {
    fn from(value: i32) -> Self {
        let sign = value < 0;
        let value = value as u128;

        let mut bn = Bignum::from(value);
        bn.sign = sign;

        bn
    }
}

#[allow(dead_code)]
fn get_test_cases() -> Vec<(u128, u128)> {
    let mut test_cases = vec![(0, 0), (0, 0xa), (0xa, 0)];
    let step_size = 0xffffffffffedcba;

    for a in (0..u64::MAX).step_by(step_size) {
        for b in (0..u64::MAX).step_by(step_size) {
            test_cases.push((a as u128, b as u128));
        }
    }
    test_cases
}

#[cfg(test)]
mod tests {
    use crate::math::bignum::Bignum;

    #[test]
    fn from_hex_string() {
        for s in [
            "0xabcdeddedbed12983075980123",
            "0xdeadbeef",
            "0x1234124124590856b",
            "0x0",
            "0x1",
            "-0xabcdeddedbed12983075980123",
            "-0xdeadbeef",
            "-0x1234124124590856b",
            "-0x1",
        ] {
            let bn = Bignum::try_from_hex_string(s).unwrap();

            assert_eq!(s, bn.to_hex_string());
        }
    }
}
