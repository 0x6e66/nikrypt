/// Internal storage in little endian
///
/// 0xabcdef00 -> Bignum([0x00, 0xef, 0xcd, 0xab])
#[derive(Debug, Clone)]
pub struct Bignum(Vec<u8>);

impl Bignum {
    pub fn new() -> Self {
        Self(vec![0u8])
    }

    pub fn from_little_endian(value: &[u8]) -> Self {
        Self(Vec::from(value))
    }

    pub fn from_big_endian(value: &[u8]) -> Self {
        let mut vec = Vec::from(value);
        vec.reverse();
        Self(vec)
    }

    pub fn to_hex_string(&self) -> String {
        if self.0.len() == 1 && self.0[0] == 0 {
            return String::from("0x0");
        }

        let mut res = String::new();
        let mut leading_zeros = true;

        for b in self.0.iter().rev() {
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

        let mut b = Bignum::from_little_endian(&vec);
        b.strip();

        Ok(b)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn strip(&mut self) {
        let mut count = 0;

        for b in self.0.iter().rev() {
            if *b != 0 {
                break;
            }
            count += 1;
        }

        self.0.resize(self.0.len() - count, 0u8);

        if self.0.is_empty() {
            self.0.push(0u8);
        }
    }

    pub fn get_bit(&self, pos: usize) -> bool {
        if pos >= self.0.len() * 8 {
            return false;
        }

        let byte = self.0[pos / 8];
        (byte >> (pos % 8)) & 1 == 1
    }

    pub fn set_bit(&mut self, pos: usize) {
        if pos >= self.0.len() * 8 {
            self.0.resize(pos / 8 + 1, 0);
        }

        self.0[pos / 8] |= 1 << (pos % 8);
    }

    pub fn unset_bit(&mut self, pos: usize) {
        if pos >= self.0.len() * 8 {
            return;
        }

        self.0[pos / 8] &= !(1 << (pos % 8));
    }

    pub fn toggle_bit(&mut self, pos: usize) {
        if pos >= self.0.len() * 8 {
            self.0.resize(pos / 8 + 1, 0);
        }

        self.0[pos / 8] ^= 1 << (pos % 8);
    }

    /// Integer division (unsigned) with remainder (https://en.wikipedia.org/wiki/Division_algorithm#Integer_division_(unsigned)_with_remainder)
    /// returns (quotient, remainder)
    pub fn div_with_remainder(&self, rhs: &Self) -> (Self, Self) {
        let mut quotient = Bignum::new();
        let mut remainder = Bignum::new();

        let (n_len, n) = (self.0.len() * 8, self);

        for i in (0..n_len).rev() {
            remainder = remainder << 1;
            if n.get_bit(i) {
                remainder.set_bit(0);
            } else {
                remainder.unset_bit(0);
            }

            if remainder >= *rhs {
                remainder = remainder - rhs.clone();
                quotient.set_bit(i);
            }
        }

        (quotient, remainder)
    }

    pub fn is_zero(&self) -> bool {
        self.0.len() == 1 && self.0[0] == 0
    }

    pub fn is_even(&self) -> bool {
        self.0[0] % 2 == 0
    }

    /// Exponentiation by squaring (https://en.wikipedia.org/wiki/Exponentiation_by_squaring)
    pub fn pow(self, other: Self) -> Self {
        let mut x = self;
        let mut n = other;

        if n.is_zero() {
            return 1.into();
        }

        let mut y = Bignum::from(1);
        while n > 1.into() {
            if !n.is_even() {
                y = x.clone() * y;
                n = n - 1.into();
            }
            x = x.clone() * x;
            n = n / 2.into();
        }

        return x * y;
    }

    pub fn mul_ref(&self, other: &Self) -> Self {
        let p = self.0.len();
        let q = other.0.len();
        let base = 256;

        let mut product = vec![0; p + q];

        for b_i in 0..q {
            let mut carry = 0;
            for a_i in 0..p {
                let mut tmp = product[a_i + b_i] as u32;
                tmp += carry + self.0[a_i] as u32 * other.0[b_i] as u32;
                carry = tmp / base;
                tmp %= base;
                product[a_i + b_i] = tmp as u8;
            }
            product[b_i + p] = carry as u8;
        }

        let mut tmp = Self(product);
        tmp.strip();

        tmp
    }

    pub fn pow_mod(self, exponent: Self, modulus: &Self) -> Self {
        let mut base = self;
        let mut exp = exponent;

        let mut t = Bignum::from(1);
        while !exp.is_zero() {
            if !exp.is_even() {
                (_, t) = Bignum::mul_ref(&t, &base).div_with_remainder(&modulus);
            }
            (_, base) = Bignum::mul_ref(&base, &base).div_with_remainder(&modulus);
            exp = exp >> 1;
        }

        let (_, r) = t.div_with_remainder(&modulus);
        r
    }

    pub fn add_ref(&self, rhs: &Self) -> Self {
        let (long, short) = match self.len() > rhs.len() {
            true => (self, rhs),
            false => (rhs, self),
        };
        let mut vec = vec![0u8; long.len()];

        let mut carry = 0;
        for i in 0..short.len() {
            let tmp = short.0[i] as u16 + long.0[i] as u16 + carry;
            carry = tmp >> 8;

            vec[i] = tmp as u8;
        }

        for i in short.len()..long.len() {
            let tmp = long.0[i] as u16 + carry;
            carry = tmp >> 8;

            vec[i] = tmp as u8;
        }

        if carry != 0 {
            vec.push(carry as u8);
        }

        Self(vec)
    }

    pub fn sub_ref(&self, rhs: &Self) -> Self {
        if self.len() < rhs.len() {
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
        for i in 0..long.len() {
            let (mut sum, mut tmp_carry) = long.0[i].overflowing_sub(carry);
            carry = tmp_carry as u8;

            if i < short.len() {
                (sum, tmp_carry) = sum.overflowing_sub(short.0[i]);
                carry += tmp_carry as u8;
            }

            vec[i] = sum;
        }

        let mut res = Bignum(vec);
        res.strip();

        res
    }
}

impl Default for Bignum {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u128> for Bignum {
    fn from(value: u128) -> Self {
        let mut res = Self(vec![
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
        ]);
        res.strip();
        res
    }
}

impl PartialEq for Bignum {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        for i in 0..self.0.len() {
            if self.0[i] != other.0[i] {
                return false;
            }
        }

        true
    }
}

impl PartialOrd for Bignum {
    fn lt(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return self.0.len().lt(&other.0.len());
        }

        for (s, o) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if s != o {
                return s.lt(o);
            }
        }

        false
    }

    fn le(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return self.0.len().lt(&other.0.len());
        }

        for (s, o) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if s != o {
                return s.lt(o);
            }
        }

        true
    }

    fn gt(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return self.0.len().gt(&other.0.len());
        }

        for (s, o) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if s != o {
                return s.gt(o);
            }
        }

        false
    }

    fn ge(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return self.0.len().gt(&other.0.len());
        }

        for (s, o) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if s != o {
                return s.gt(o);
            }
        }

        true
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.0.len() != other.0.len() {
            return Some(self.0.len().cmp(&other.0.len()));
        }

        for (s, o) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if s != o {
                return Some(s.cmp(o));
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl std::ops::Shr<usize> for Bignum {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        let (new_len, _) = self.0.len().overflowing_sub(rhs / 8);
        let shift = (rhs % 8) as u8;

        self.0.resize(new_len, 0);

        let mut carry = 0;
        for b in self.0.iter_mut().rev() {
            let tmp_carry = *b << (8 - shift);
            *b >>= shift;
            *b |= carry;
            carry = tmp_carry;
        }

        self.strip();

        self
    }
}

impl std::ops::Shl<usize> for Bignum {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        let shift = (rhs % 8) as u8;

        self.0.push(0);

        for _ in 0..(rhs / 8) {
            self.0.insert(0, 0);
        }

        let mut carry = 0;
        for b in self.0.iter_mut() {
            let tmp_carry = *b >> (8 - shift);
            *b <<= shift;
            *b |= carry;
            carry = tmp_carry;
        }

        self.strip();

        self
    }
}

impl std::ops::Add for Bignum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_ref(&rhs)
    }
}

impl std::ops::Sub for Bignum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_ref(&rhs)
    }
}

// Long Multiplication (https://en.wikipedia.org/wiki/Multiplication_algorithm#Long_multiplication)
impl std::ops::Mul for Bignum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Bignum::mul_ref(&self, &rhs)
    }
}

impl std::ops::Div for Bignum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let (q, _) = self.div_with_remainder(&rhs);
        q
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUM_PAIRS: [(u128, u128); 7] = [
        (0xaabb0000, 0x0000ccdd),
        (0xffff, 0xffff),
        (0x0, 0x0),
        (0x0, 0x1),
        (0xabcedefabcdef, 0xabcedefabcdef),
        (0xabcedef, 0xabcedefabcdef),
        (0xabcedefabcdef, 0xabcedef),
    ];

    const NUM_PAIRS2: [(u128, usize); 4] = [
        (0xffff, 12),
        (0xabcedefabcdef, 5),
        (0xffffff, 10),
        (0xff, 15),
    ];

    #[test]
    fn addition() {
        for (a, b) in NUM_PAIRS {
            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            let res = Bignum::from(a + b);
            let res_big = big_a + big_b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn subtraction() {
        for (a, b) in NUM_PAIRS {
            let (a, b) = match a >= b {
                true => (a, b),
                false => (b, a),
            };

            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            let res = Bignum::from(a - b);
            let res_big = big_a - big_b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    #[should_panic]
    fn subtraction_panic() {
        for (a, b) in NUM_PAIRS {
            let (a, b) = match a >= b {
                false => (a, b),
                true => (b, a),
            };

            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            let _res_big = big_a - big_b;
        }
    }

    #[test]
    fn multiplication() {
        for (a, b) in NUM_PAIRS {
            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            let res = Bignum::from(a * b);
            let res_big = big_a * big_b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn division_with_remainder() {
        for (a, b) in NUM_PAIRS2 {
            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b as u128);

            let (big_q, big_r) = Bignum::div_with_remainder(&big_a, &big_b);
            let q = Bignum::from(a / b as u128);
            let r = Bignum::from(a % b as u128);

            assert_eq!(big_q, q);
            assert_eq!(big_r, r);
        }
    }

    #[test]
    fn pow() {
        for (a, b) in [(35, 7)] {
            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            let res = Bignum::from(a.pow(b as u32));
            let res_big = big_a.pow(big_b);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn comparison() {
        for (a, b) in NUM_PAIRS {
            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            let res = a.partial_cmp(&b);
            let res_big = big_a.partial_cmp(&big_b);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn shift_right() {
        for (a, b) in NUM_PAIRS2 {
            let big_a = Bignum::from(a);

            let (tmp, _) = a.overflowing_shr(b as u32);
            let res = Bignum::from(tmp);
            let res_big = big_a >> b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn shift_left() {
        for (a, b) in NUM_PAIRS2 {
            let big_a = Bignum::from(a);

            let tmp = a << b as u32;
            let res = Bignum::from(tmp);
            let res_big = big_a << b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn get_bit() {
        for (a, b) in NUM_PAIRS2 {
            let big_a = Bignum::from(a);

            let res = (a >> b) & 1 == 1;
            let res_big = big_a.get_bit(b);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn set_bit() {
        for (mut a, b) in NUM_PAIRS2 {
            let mut big_a = Bignum::from(a);
            big_a.set_bit(b);

            a |= 1 << b;
            let a = Bignum::from(a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn unset_bit() {
        for (mut a, b) in NUM_PAIRS2 {
            let mut big_a = Bignum::from(a);
            big_a.unset_bit(b);

            a &= !(1 << b);
            let a = Bignum::from(a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn toggle_bit() {
        for (mut a, b) in NUM_PAIRS2 {
            let mut big_a = Bignum::from(a);
            big_a.toggle_bit(b);

            a ^= 1 << b;
            let a = Bignum::from(a);

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
            let bn = Bignum::try_from_hex_string(s).unwrap();

            assert_eq!(s, bn.to_hex_string());
        }
    }
}
