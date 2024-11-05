use std::io::Read;
/// Internal storage in little endian
///
/// 0xabcdef00 -> Bignum([0x00, 0xef, 0xcd, 0xab])
#[derive(Debug, Clone)]
pub struct UnsignedBignum {
    digits: Vec<u8>,
}

impl UnsignedBignum {
    pub fn new() -> Self {
        Self { digits: vec![0u8] }
    }

    pub fn from_little_endian(value: &[u8]) -> Self {
        let mut bn = Self {
            digits: Vec::from(value),
        };

        bn.strip();
        bn
    }

    pub fn from_big_endian(value: &[u8]) -> Self {
        let mut vec = Vec::from(value);
        vec.reverse();

        let mut bn = Self { digits: vec };
        bn.strip();
        bn
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

            res.push_str(&format!("{:02x}", b));
        }

        if let Some(tmp) = res.strip_prefix('0') {
            res = tmp.to_string();
        }

        format!("0x{}", res)
    }

    pub fn try_from_hex_string(t: &str) -> Result<Self, std::num::ParseIntError> {
        let s = t.trim_start_matches("0x");

        let mut vec = vec![];

        let len = s.len();
        for i in 0..len / 2 {
            let b = &s[len - (2 * i + 2)..len - 2 * i];
            let b = u8::from_str_radix(b, 16)?;
            vec.push(b);
        }

        if len % 2 != 0 {
            let b = &s[0..1];
            let b = u8::from_str_radix(b, 16)?;
            vec.push(b);
        }

        let mut b = UnsignedBignum::from_little_endian(&vec);
        b.strip();

        Ok(b)
    }

    pub fn len(&self) -> usize {
        self.digits.len()
    }

    fn strip(&mut self) {
        let mut count = 0;

        for b in self.digits.iter().rev() {
            if *b != 0 {
                break;
            }
            count += 1;
        }

        self.digits.resize(self.digits.len() - count, 0u8);

        if self.digits.is_empty() {
            self.digits.push(0u8);
        }
    }

    pub fn get_bit(&self, pos: usize) -> bool {
        if pos >= self.digits.len() * 8 {
            return false;
        }

        let byte = self.digits[pos / 8];
        (byte >> (pos % 8)) & 1 == 1
    }

    pub fn set_bit(&mut self, pos: usize) {
        if pos >= self.digits.len() * 8 {
            self.digits.resize(pos / 8 + 1, 0);
        }

        self.digits[pos / 8] |= 1 << (pos % 8);
    }

    pub fn unset_bit(&mut self, pos: usize) {
        if pos >= self.digits.len() * 8 {
            return;
        }

        self.digits[pos / 8] &= !(1 << (pos % 8));
    }

    pub fn toggle_bit(&mut self, pos: usize) {
        if pos >= self.digits.len() * 8 {
            self.digits.resize(pos / 8 + 1, 0);
        }

        self.digits[pos / 8] ^= 1 << (pos % 8);
    }

    /// Integer division (unsigned) with remainder (https://en.wikipedia.org/wiki/Division_algorithm#Integer_division_(unsigned)_with_remainder)
    /// returns (quotient, remainder)
    pub fn div_with_remainder(&self, rhs: &Self) -> (Self, Self) {
        let mut quotient = Self::new();
        let mut remainder = Self::new();

        let (n_len, n) = (self.digits.len() * 8, self);

        for i in (0..n_len).rev() {
            remainder = remainder << 1;
            if n.get_bit(i) {
                remainder.set_bit(0);
            } else {
                remainder.unset_bit(0);
            }

            if remainder >= *rhs {
                remainder = remainder.sub_ref(rhs);
                quotient.set_bit(i);
            }
        }

        (quotient, remainder)
    }

    pub fn is_zero(&self) -> bool {
        self.digits.len() == 1 && self.digits[0] == 0
    }

    pub fn is_empty(&self) -> bool {
        self.is_zero()
    }

    pub fn is_even(&self) -> bool {
        self.digits[0] % 2 == 0
    }

    /// Exponentiation by squaring (https://en.wikipedia.org/wiki/Exponentiation_by_squaring)
    pub fn pow(self, other: Self) -> Self {
        let mut x = self;
        let mut n = other;

        let one = 1.into();
        let two = 2.into();

        if n.is_zero() {
            return one;
        }

        let mut y = one.clone();
        while n > 1.into() {
            if !n.is_even() {
                y = x.mul_ref(&y);
                n = n.sub_ref(&one);
            }
            x = x.mul_ref(&x);
            (n, _) = n.div_with_remainder(&two);
        }

        x * y
    }

    pub fn mul_ref(&self, other: &Self) -> Self {
        let p = self.digits.len();
        let q = other.digits.len();
        let base = 256;

        let mut product = vec![0; p + q];

        for b_i in 0..q {
            let mut carry = 0;
            for a_i in 0..p {
                let mut tmp = product[a_i + b_i] as u16;
                tmp += carry + self.digits[a_i] as u16 * other.digits[b_i] as u16;
                carry = tmp / base;
                tmp %= base;
                product[a_i + b_i] = tmp as u8;
            }
            product[b_i + p] = carry as u8;
        }

        let mut tmp = Self { digits: product };
        tmp.strip();

        tmp
    }

    pub fn pow_mod(self, exponent: Self, modulus: &Self) -> Self {
        let mut base = self;
        let mut exp = exponent;

        let mut t = Self::from(1);
        while !exp.is_zero() {
            if !exp.is_even() {
                (_, t) = Self::mul_ref(&t, &base).div_with_remainder(modulus);
            }
            (_, base) = Self::mul_ref(&base, &base).div_with_remainder(modulus);
            exp = exp >> 1;
        }

        let (_, r) = t.div_with_remainder(modulus);
        r
    }

    pub fn add_ref(&self, rhs: &Self) -> Self {
        let (long, short) = match self.len() > rhs.len() {
            true => (self, rhs),
            false => (rhs, self),
        };
        let mut vec = vec![0u8; long.len()];

        let mut carry = 0;
        for (i, e) in vec.iter_mut().enumerate() {
            let mut tmp = long.digits[i] as u16 + carry;
            if i < short.len() {
                tmp += short.digits[i] as u16;
            }
            carry = tmp >> 8;

            *e = tmp as u8;
        }

        if carry != 0 {
            vec.push(carry as u8);
        }

        Self { digits: vec }
    }

    pub fn sub_ref(&self, rhs: &Self) -> Self {
        if self < rhs {
            panic!(
                "Result of subtraction would be negative.\nlhs: {}\nrhs: {}",
                self.to_hex_string(),
                rhs.to_hex_string()
            );
        }

        let (long, short) = match self > rhs {
            true => (self, rhs),
            false => (rhs, self),
        };
        let mut vec = vec![0u8; long.len()];

        let mut carry = 0;
        for (i, e) in vec.iter_mut().enumerate() {
            let (mut sum, mut tmp_carry) = long.digits[i].overflowing_sub(carry);
            carry = tmp_carry as u8;

            if i < short.len() {
                (sum, tmp_carry) = sum.overflowing_sub(short.digits[i]);
                carry += tmp_carry as u8;
            }

            *e = sum;
        }

        let mut res = Self { digits: vec };
        res.strip();

        res
    }

    /// Generate random number with `n` bytes
    pub fn rand(n: usize) -> Self {
        if n == 0 {
            panic!("Can't create Bignum with 0 bytes. n has to be > 0");
        }
        let mut f = std::fs::File::open("/dev/urandom").expect("Can't open file /dev/urandom");
        let mut buf = vec![0; n];
        f.read_exact(&mut buf)
            .expect("Can't read from file /dev/urandom");
        Self { digits: buf }
    }
}

impl Default for UnsignedBignum {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u128> for UnsignedBignum {
    fn from(value: u128) -> Self {
        let mut res = Self {
            digits: vec![
                value as u8,
                (value >> 8) as u8,
                (value >> (2 * 8)) as u8,
                (value >> (3 * 8)) as u8,
                (value >> (4 * 8)) as u8,
                (value >> (5 * 8)) as u8,
                (value >> (6 * 8)) as u8,
                (value >> (7 * 8)) as u8,
                (value >> (8 * 8)) as u8,
                (value >> (9 * 8)) as u8,
                (value >> (10 * 8)) as u8,
                (value >> (11 * 8)) as u8,
                (value >> (12 * 8)) as u8,
                (value >> (13 * 8)) as u8,
                (value >> (14 * 8)) as u8,
                (value >> (15 * 8)) as u8,
            ],
        };
        res.strip();
        res
    }
}

impl PartialEq for UnsignedBignum {
    fn eq(&self, other: &Self) -> bool {
        if self.digits.len() != other.digits.len() {
            return false;
        }

        for i in 0..self.digits.len() {
            if self.digits[i] != other.digits[i] {
                return false;
            }
        }

        true
    }
}

impl PartialOrd for UnsignedBignum {
    fn lt(&self, other: &Self) -> bool {
        if self.digits.len() != other.digits.len() {
            return self.digits.len().lt(&other.digits.len());
        }

        for (s, o) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if s != o {
                return s.lt(o);
            }
        }

        false
    }

    fn le(&self, other: &Self) -> bool {
        if self.digits.len() != other.digits.len() {
            return self.digits.len().lt(&other.digits.len());
        }

        for (s, o) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if s != o {
                return s.lt(o);
            }
        }

        true
    }

    fn gt(&self, other: &Self) -> bool {
        if self.digits.len() != other.digits.len() {
            return self.digits.len().gt(&other.digits.len());
        }

        for (s, o) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if s != o {
                return s.gt(o);
            }
        }

        false
    }

    fn ge(&self, other: &Self) -> bool {
        if self.digits.len() != other.digits.len() {
            return self.digits.len().gt(&other.digits.len());
        }

        for (s, o) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if s != o {
                return s.gt(o);
            }
        }

        true
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.digits.len() != other.digits.len() {
            return Some(self.digits.len().cmp(&other.digits.len()));
        }

        for (s, o) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if s != o {
                return Some(s.cmp(o));
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl std::ops::Shr<usize> for UnsignedBignum {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        let new_len = self.len() - self.digits.len().saturating_sub(rhs / 8);
        let bit_shift = (rhs % 8) as u8;

        for _ in 0..new_len {
            self.digits.remove(0);
        }

        if bit_shift == 0 {
            self.strip();
            return self;
        }

        let mut carry = 0;
        for b in self.digits.iter_mut().rev() {
            let tmp_carry = *b << (8 - bit_shift);
            *b >>= bit_shift;
            *b |= carry;
            carry = tmp_carry;
        }

        self.strip();

        self
    }
}

impl std::ops::Shl<usize> for UnsignedBignum {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        let byte_shift = rhs / 8;
        let shift = (rhs % 8) as u8;

        for _ in 0..byte_shift {
            self.digits.insert(0, 0);
        }

        if shift == 0 {
            return self;
        }

        self.digits.push(0);

        let mut carry = 0;
        for b in self.digits.iter_mut() {
            let tmp_carry = *b >> (8 - shift);
            *b <<= shift;
            *b |= carry;
            carry = tmp_carry;
        }

        self.strip();

        self
    }
}

impl std::ops::Add for UnsignedBignum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_ref(&rhs)
    }
}

impl std::ops::Sub for UnsignedBignum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_ref(&rhs)
    }
}

// Long Multiplication (https://en.wikipedia.org/wiki/Multiplication_algorithm#Long_multiplication)
impl std::ops::Mul for UnsignedBignum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::mul_ref(&self, &rhs)
    }
}

impl std::ops::Div for UnsignedBignum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let (q, _) = self.div_with_remainder(&rhs);
        q
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_cases() -> Vec<(u128, u128)> {
        let mut test_cases: Vec<(u128, u128)> = vec![(0, 0xa), (0xa, 0), (0, 0)];
        for a in (0..0xabcedef).step_by(300_000) {
            for b in (0..0xabcedef).step_by(300_000) {
                test_cases.push((a, b));
            }
        }

        test_cases
    }

    #[test]
    fn addition() {
        for (a, b) in get_test_cases() {
            let big_a = UnsignedBignum::from(a);
            let big_b = UnsignedBignum::from(b);

            let res = UnsignedBignum::from(a + b);
            let res_big = big_a + big_b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn subtraction() {
        for (a, b) in get_test_cases() {
            let (a, b) = match a >= b {
                true => (a, b),
                false => (b, a),
            };

            let big_a = UnsignedBignum::from(a);
            let big_b = UnsignedBignum::from(b);

            let res = UnsignedBignum::from(a - b);
            let res_big = big_a - big_b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    #[should_panic]
    fn subtraction_panic() {
        for (a, b) in get_test_cases() {
            let (a, b) = match a > b {
                false => (a, b),
                true => (b, a),
            };

            let big_a = UnsignedBignum::from(a);
            let big_b = UnsignedBignum::from(b);

            // should panic here
            let _res_big = big_a - big_b;
        }
    }

    #[test]
    fn multiplication() {
        for (a, b) in get_test_cases() {
            let big_a = UnsignedBignum::from(a);
            let big_b = UnsignedBignum::from(b);

            let res = UnsignedBignum::from(a * b);
            let res_big = big_a * big_b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn division_with_remainder() {
        for (a, b) in get_test_cases() {
            if b == 0 {
                continue;
            }

            let big_a = UnsignedBignum::from(a);
            let big_b = UnsignedBignum::from(b as u128);

            let (big_q, big_r) = UnsignedBignum::div_with_remainder(&big_a, &big_b);
            let q = UnsignedBignum::from(a / b as u128);
            let r = UnsignedBignum::from(a % b as u128);

            assert_eq!(big_q, q);
            assert_eq!(big_r, r);
        }
    }

    #[test]
    fn pow() {
        let mut test_cases: Vec<(u128, u128)> = vec![(0, 0xa), (0xa, 0), (0, 0)];
        for a in 0..20 {
            for b in 0..20 {
                test_cases.push((a, b));
            }
        }

        for (a, b) in test_cases {
            let big_a = UnsignedBignum::from(a);
            let big_b = UnsignedBignum::from(b);

            let res = UnsignedBignum::from(a.pow(b as u32));
            let res_big = big_a.pow(big_b);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn comparison() {
        for (a, b) in get_test_cases() {
            let big_a = UnsignedBignum::from(a);
            let big_b = UnsignedBignum::from(b);

            let res = a.eq(&b);
            let res_big = big_a.eq(&big_b);
            assert_eq!(res, res_big);

            let res = a.lt(&b);
            let res_big = big_a.lt(&big_b);
            assert_eq!(res, res_big);

            let res = a.le(&b);
            let res_big = big_a.le(&big_b);
            assert_eq!(res, res_big);

            let res = a.gt(&b);
            let res_big = big_a.gt(&big_b);
            assert_eq!(res, res_big);

            let res = a.ge(&b);
            let res_big = big_a.ge(&big_b);
            assert_eq!(res, res_big);

            let res = a.partial_cmp(&b);
            let res_big = big_a.partial_cmp(&big_b);
            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn shift_right() {
        let base = 0xabcedef;
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..127 {
            test_cases.push((base, i));
        }

        for (a, b) in test_cases {
            let big_a = UnsignedBignum::from(a);

            let res = UnsignedBignum::from(a >> b);
            let res_big = big_a >> b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn shift_left() {
        let base = 0xabcedef;
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..95 {
            test_cases.push((base, i));
        }

        for (a, b) in test_cases {
            let big_a = UnsignedBignum::from(a);

            let res = UnsignedBignum::from(a << b);
            let res_big = big_a << b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn get_bit() {
        let base = 0xabcedef;
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..127 {
            test_cases.push((base, i));
        }

        for (a, b) in test_cases {
            let big_a = UnsignedBignum::from(a);
            let res_big = big_a.get_bit(b);

            let res = (a >> b) & 1 == 1;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn set_bit() {
        let base = 0xabcedef;
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..127 {
            test_cases.push((base, i));
        }

        for (mut a, b) in test_cases {
            let mut big_a = UnsignedBignum::from(a);
            big_a.set_bit(b);

            a |= 1 << b;
            let a = UnsignedBignum::from(a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn unset_bit() {
        let base = 0xabcedef;
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..127 {
            test_cases.push((base, i));
        }

        for (mut a, b) in test_cases {
            let mut big_a = UnsignedBignum::from(a);
            big_a.unset_bit(b);

            a &= !(1 << b);
            let a = UnsignedBignum::from(a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn toggle_bit() {
        let base = 0xabcedef;
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..127 {
            test_cases.push((base, i));
        }

        for (mut a, b) in test_cases {
            let mut big_a = UnsignedBignum::from(a);
            big_a.toggle_bit(b);

            a ^= 1 << b;
            let a = UnsignedBignum::from(a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn from_hex_string() {
        for s in [
            "0xabcdeddedbed12983075980123",
            "0xdeadbeef",
            "0x1234124124590856b",
            "0x0",
            "0x1",
        ] {
            let bn = UnsignedBignum::try_from_hex_string(s).unwrap();

            assert_eq!(s, bn.to_hex_string());
        }
    }
}
