use super::unsigned_bignum_fast::UnsignedBignumFast;

#[derive(Debug, Clone)]
pub struct SignedBignumFast<const NUM_BYTES: usize> {
    pub(crate) digits: [u8; NUM_BYTES],
    pub(crate) pos: usize,
    pub(crate) sign: bool,
}

fn calc_pos(length: usize) -> usize {
    if length <= 2 {
        0
    } else if length % 2 == 0 {
        length / 2 - 1
    } else {
        length / 2
    }
}

impl<const NUM_BYTES: usize> SignedBignumFast<NUM_BYTES> {
    pub fn new() -> Self {
        SignedBignumFast::zero()
    }

    pub fn zero() -> Self {
        Self {
            digits: [0; NUM_BYTES],
            pos: 0,
            sign: false,
        }
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

    pub fn from_big_endian(value: &[u8]) -> Option<Self> {
        if value.len() > NUM_BYTES {
            return None;
        }

        let mut bignum = Self::new();

        let mut pos_last_non_zero = 0;
        for (i, e) in value.iter().rev().enumerate() {
            if *e != 0 {
                pos_last_non_zero = i;
            }
            bignum.digits[i] = *e;
        }

        bignum.pos = calc_pos(value.len() * 2);
        if bignum.pos > 0 {
            bignum.pos = pos_last_non_zero;
        }

        Some(bignum)
    }

    pub fn from_little_endian(value: &[u8]) -> Option<Self> {
        if value.len() > NUM_BYTES {
            return None;
        }

        let mut bignum = Self::new();

        let mut pos_last_non_zero = 0;
        for (i, e) in value.iter().enumerate() {
            if *e != 0 {
                pos_last_non_zero = i;
            }
            bignum.digits[i] = *e;
        }

        bignum.pos = calc_pos(value.len() * 2);
        if bignum.pos > 0 {
            bignum.pos = pos_last_non_zero;
        }

        Some(bignum)
    }

    pub fn try_from_hex_string(s: &str) -> Result<Self, std::num::ParseIntError> {
        let s = s.trim_start_matches("0x");
        let s = s.trim_start_matches('0');

        let mut bignum = Self::new();
        let len = s.len();

        bignum.pos = calc_pos(len);

        for i in 0..len / 2 {
            let b = &s[len - (2 * i + 2)..len - 2 * i];
            let b = u8::from_str_radix(b, 16)?;
            bignum.digits[i] = b;
        }

        if len % 2 != 0 {
            let b = &s[0..1];
            let b = u8::from_str_radix(b, 16)?;
            bignum.digits[len / 2] = b;
        }

        Ok(bignum)
    }

    pub fn to_hex_string(&self) -> String {
        if self.pos == 0 && self.digits[0] == 0 {
            return String::from("0x0");
        }

        let mut res = String::new();
        if self.sign {
            res.push('-');
        }
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

    pub fn get_bit(&self, pos: usize) -> bool {
        let byte_pos = pos / 8;
        if pos >= NUM_BYTES * 8 {
            panic!("Bit index out of bounds. Max index is {}", NUM_BYTES * 8);
        }

        let byte = self.digits[byte_pos];
        (byte >> (pos % 8)) & 1 == 1
    }

    pub fn set_bit(&mut self, pos: usize) {
        let byte_pos = pos / 8;
        if byte_pos >= NUM_BYTES {
            panic!("Bit index out of bounds. Max index is {}", NUM_BYTES * 8);
        }

        self.digits[byte_pos] |= 1 << (pos % 8);

        if byte_pos > self.pos {
            self.pos = byte_pos;
        }
    }

    pub fn unset_bit(&mut self, pos: usize) {
        let byte_pos = pos / 8;
        if byte_pos >= NUM_BYTES {
            panic!("Bit index out of bounds. Max index is {}", NUM_BYTES * 8);
        }

        self.digits[byte_pos] &= !(1 << (pos % 8));

        for (i, e) in self.digits[0..self.len()].iter().enumerate().rev() {
            if *e != 0 || i == 0 {
                self.pos = i;
                return;
            }
        }
    }

    pub fn toggle_bit(&mut self, pos: usize) {
        let byte_pos = pos / 8;
        if byte_pos >= NUM_BYTES {
            panic!("Bit index out of bounds. Max index is {}", NUM_BYTES * 8);
        }

        self.digits[byte_pos] ^= 1 << (pos % 8);

        if self.digits[byte_pos] != 0 {
            // set bit
            if byte_pos > self.pos {
                self.pos = byte_pos;
            }
        } else {
            // unset bit
            for (i, e) in self.digits[0..self.len()].iter().enumerate().rev() {
                if *e != 0 || i == 0 {
                    self.pos = i;
                    return;
                }
            }
        }
    }

    fn ge_internal(&self, other: &Self) -> bool {
        if self.pos != other.pos {
            return self.pos.gt(&other.pos);
        }

        for (s, o) in self.digits[0..self.len()]
            .iter()
            .rev()
            .zip(other.digits[0..self.len()].iter().rev())
        {
            if s != o {
                return s.gt(o);
            }
        }

        true
    }

    fn gt_internal(&self, other: &Self) -> bool {
        if self.pos != other.pos {
            return self.pos.gt(&other.pos);
        }

        for (s, o) in self.digits[0..self.len()]
            .iter()
            .rev()
            .zip(other.digits[0..self.len()].iter().rev())
        {
            if s != o {
                return s.gt(o);
            }
        }

        false
    }

    fn add_ref(&self, rhs: &Self) -> Self {
        match (self.sign, rhs.sign) {
            // (x)  +  (y) => x + y
            (false, false) => Self::add_ref_internal(self, rhs),
            // (-x) + (y)  => y - x
            (true, false) => {
                if self.eq(rhs) {
                    Self::new()
                } else if rhs.gt_internal(self) {
                    Self::sub_ref_internal(rhs, self)
                } else {
                    let mut tmp = Self::sub_ref_internal(self, rhs);
                    tmp.sign = true;
                    tmp
                }
            }
            // (x)  + (-y) => x - y
            (false, true) => {
                if self.eq(rhs) {
                    Self::new()
                } else if self.gt_internal(rhs) {
                    Self::sub_ref_internal(self, rhs)
                } else {
                    let mut tmp = Self::sub_ref_internal(rhs, self);
                    tmp.sign = true;
                    tmp
                }
            }
            // (-x) + (-y) => - (x + y)
            (true, true) => {
                let mut tmp = Self::add_ref_internal(self, rhs);
                tmp.sign = true;
                tmp
            }
        }
    }

    fn add_ref_internal(&self, rhs: &Self) -> Self {
        let (long, short) = match self.pos > rhs.pos {
            true => (self, rhs),
            false => (rhs, self),
        };

        let mut bignum = SignedBignumFast::new();
        bignum.pos = long.pos;

        let mut carry = 0;
        for i in 0..long.len() {
            let mut tmp = long.digits[i] as u16 + carry;
            if i < short.len() {
                tmp += short.digits[i] as u16;
            }
            carry = tmp >> 8;
            bignum.digits[i] = tmp as u8;
        }

        if carry != 0 {
            if bignum.len() == NUM_BYTES {
                panic!("Attempted addition with overflow");
            }
            bignum.digits[bignum.len()] = carry as u8;
            bignum.pos += 1;
        }

        bignum
    }

    pub fn sub_ref(&self, rhs: &Self) -> Self {
        match (self.sign, rhs.sign) {
            // (x)  -  (y) => x - y
            (false, false) => {
                if self.eq(rhs) {
                    Self::new()
                } else if self.gt_internal(rhs) {
                    Self::sub_ref_internal(self, rhs)
                } else {
                    let mut tmp = Self::sub_ref_internal(rhs, self);
                    tmp.sign = true;
                    tmp
                }
            }
            // (-x) - (y)  => - (x + y)
            (true, false) => {
                let mut tmp = Self::add_ref_internal(self, rhs);
                tmp.sign = true;
                tmp
            }
            // (x)  - (-y) => x + y
            (false, true) => Self::add_ref_internal(self, rhs),
            // (-x) - (-y) => y - x
            (true, true) => {
                if self.eq(rhs) {
                    Self::new()
                } else if rhs.gt_internal(self) {
                    Self::sub_ref_internal(rhs, self)
                } else {
                    let mut tmp = Self::sub_ref_internal(self, rhs);
                    tmp.sign = true;
                    tmp
                }
            }
        }
    }

    fn sub_ref_internal(&self, rhs: &Self) -> Self {
        match self.partial_cmp(rhs) {
            Some(std::cmp::Ordering::Equal) => return SignedBignumFast::zero(),
            _ => (),
        }

        let (long, short) = (self, rhs);
        let mut bignum = SignedBignumFast::new();

        let mut carry = 0;
        let mut pos_last_non_zero = 0;
        for i in 0..long.len() {
            let (mut sum, mut tmp_carry) = long.digits[i].overflowing_sub(carry);
            carry = tmp_carry as u8;

            if i < short.len() {
                (sum, tmp_carry) = sum.overflowing_sub(short.digits[i]);
                carry += tmp_carry as u8;
            }

            if sum != 0 {
                pos_last_non_zero = i;
            }

            bignum.digits[i] = sum;
        }
        bignum.pos = pos_last_non_zero;

        bignum
    }

    pub fn mul_ref(&self, other: &Self) -> Self {
        let p = self.len();
        let q = other.len();
        let base = 256;

        if p + q > NUM_BYTES {
            panic!("Attempted multiplication with overflow");
        }

        let mut bignum = SignedBignumFast::new();

        let mut pos_last_non_zero = 0;
        for b_i in 0..q {
            let mut carry = 0;
            for a_i in 0..p {
                let mut tmp = bignum.digits[a_i + b_i] as u16;
                tmp += carry + self.digits[a_i] as u16 * other.digits[b_i] as u16;
                carry = tmp / base;
                tmp %= base;
                bignum.digits[a_i + b_i] = tmp as u8;
                if tmp != 0 {
                    pos_last_non_zero = a_i + b_i;
                }
            }
            if carry != 0 {
                pos_last_non_zero = b_i + p;
            }
            bignum.digits[b_i + p] = carry as u8;
        }

        bignum.pos = pos_last_non_zero;

        bignum.sign = match bignum.is_zero() {
            true => false,
            false => (self.sign && !other.sign) || (!self.sign && other.sign),
        };

        bignum
    }

    pub fn div_with_remainder(&self, rhs: &Self) -> (Self, Self) {
        if self.is_zero() {
            return (Self::new(), Self::new());
        } else if rhs.is_zero() {
            panic!(
                "Attepted division by zero.\nlhs: {}\nrhs: {}",
                self.to_hex_string(),
                rhs.to_hex_string()
            );
        }

        let mut q = SignedBignumFast::new();
        let mut r = SignedBignumFast::new();

        let (n_len, n) = (self.len() * 8, self);

        for i in (0..n_len).rev() {
            r = r << 1;
            if n.get_bit(i) {
                r.set_bit(0);
            } else {
                r.unset_bit(0);
            }

            if r.ge_internal(rhs) {
                r = r.sub_ref_internal(rhs);
                q.set_bit(i);
            }
        }

        q.sign = match q.is_zero() {
            true => false,
            false => (self.sign && !rhs.sign) || (!self.sign && rhs.sign),
        };
        r.sign = match r.is_zero() {
            true => false,
            false => match (self.sign, rhs.sign) {
                (false, false) => false,
                (true, false) => true,
                (false, true) => false,
                (true, true) => true,
            },
        };

        (q, r)
    }

    pub fn pow_mod(self, exponent: Self, modulus: &Self) -> Self {
        if exponent.sign {
            panic!("Can't exponentiate with negative exponent");
        }
        let sign = match (self.sign, exponent.is_even()) {
            (false, _) => false,
            (true, false) => true,
            (true, true) => false,
        };
        let mut base = self;
        let mut exp = exponent;

        let mut t = SignedBignumFast::from(1);
        while !exp.is_zero() {
            if !exp.is_even() {
                (_, t) = SignedBignumFast::mul_ref(&t, &base).div_with_remainder(modulus);
            }
            (_, base) = SignedBignumFast::mul_ref(&base, &base).div_with_remainder(modulus);
            exp = exp >> 1;
        }

        let (_, mut r) = t.div_with_remainder(modulus);
        r.sign = sign;

        r
    }
}

impl<const NUM_BYTES: usize> Default for SignedBignumFast<NUM_BYTES> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const NUM_BYTES: usize> PartialEq for SignedBignumFast<NUM_BYTES> {
    fn eq(&self, other: &Self) -> bool {
        if self.pos != other.pos {
            return false;
        }

        for (s, o) in self.digits[0..self.len()]
            .iter()
            .rev()
            .zip(other.digits[0..self.len()].iter().rev())
        {
            if s != o {
                return false;
            }
        }

        true
    }
}

impl<const NUM_BYTES: usize> PartialOrd for SignedBignumFast<NUM_BYTES> {
    fn lt(&self, other: &Self) -> bool {
        let inverse = match (self.sign, other.sign) {
            (true, false) => return true,
            (false, true) => return false,
            (false, false) => false,
            (true, true) => true,
        };

        if self.pos != other.pos {
            return match inverse {
                false => self.pos.lt(&other.pos),
                true => other.pos.lt(&self.pos),
            };
        }

        for (s, o) in self.digits[0..self.len()]
            .iter()
            .rev()
            .zip(other.digits[0..self.len()].iter().rev())
        {
            if s != o {
                return match inverse {
                    false => s.lt(o),
                    true => o.lt(s),
                };
            }
        }

        false
    }

    fn le(&self, other: &Self) -> bool {
        let inverse = match (self.sign, other.sign) {
            (true, false) => return true,
            (false, true) => return false,
            (false, false) => false,
            (true, true) => true,
        };

        if self.pos != other.pos {
            return match inverse {
                false => self.pos.lt(&other.pos),
                true => other.pos.lt(&self.pos),
            };
        }

        for (s, o) in self.digits[0..self.len()]
            .iter()
            .rev()
            .zip(other.digits[0..self.len()].iter().rev())
        {
            if s != o {
                return match inverse {
                    false => s.lt(o),
                    true => o.lt(s),
                };
            }
        }

        true
    }

    fn gt(&self, other: &Self) -> bool {
        let inverse = match (self.sign, other.sign) {
            (true, false) => return false,
            (false, true) => return true,
            (false, false) => false,
            (true, true) => true,
        };

        if self.pos != other.pos {
            return match inverse {
                false => self.pos.gt(&other.pos),
                true => other.pos.gt(&self.pos),
            };
        }

        for (s, o) in self.digits[0..self.len()]
            .iter()
            .rev()
            .zip(other.digits[0..self.len()].iter().rev())
        {
            if s != o {
                return match inverse {
                    false => s.gt(o),
                    true => o.gt(s),
                };
            }
        }

        false
    }

    fn ge(&self, other: &Self) -> bool {
        let inverse = match (self.sign, other.sign) {
            (true, false) => return false,
            (false, true) => return true,
            (false, false) => false,
            (true, true) => true,
        };

        if self.pos != other.pos {
            return match inverse {
                false => self.pos.gt(&other.pos),
                true => other.pos.gt(&self.pos),
            };
        }

        for (s, o) in self.digits[0..self.len()]
            .iter()
            .rev()
            .zip(other.digits[0..self.len()].iter().rev())
        {
            if s != o {
                return match inverse {
                    false => s.gt(o),
                    true => o.gt(s),
                };
            }
        }

        true
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let inverse = match (self.sign, other.sign) {
            (true, false) => return Some(std::cmp::Ordering::Less),
            (false, true) => return Some(std::cmp::Ordering::Greater),
            (false, false) => false,
            (true, true) => true,
        };

        if self.pos != other.pos {
            return match inverse {
                false => Some(self.pos.cmp(&other.pos)),
                true => Some(other.pos.cmp(&self.pos)),
            };
        }

        for (s, o) in self.digits[0..self.len()]
            .iter()
            .rev()
            .zip(other.digits[0..self.len()].iter().rev())
        {
            if s != o {
                return match inverse {
                    false => Some(s.cmp(o)),
                    true => Some(o.cmp(s)),
                };
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl<const NUM_BYTES: usize> std::ops::Shr<usize> for SignedBignumFast<NUM_BYTES> {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        let shift = (rhs % 8) as u8;
        let bytes_shift = rhs / 8;

        if bytes_shift >= self.len() {
            return Self::zero();
        }

        for i in 0..self.len() {
            self.digits[i] = self.digits[i + bytes_shift];
        }

        if shift == 0 {
            self.pos -= bytes_shift;
            return self;
        }

        let mut carry = 0;
        for i in (0..self.len()).rev() {
            let tmp_carry = (self.digits[i] as u16) << (8 - shift);
            self.digits[i] >>= shift;
            self.digits[i] |= carry;
            carry = tmp_carry as u8;
        }

        self.pos -= bytes_shift;
        if self.digits[self.pos] == 0 && self.pos > 0 {
            self.pos -= 1;
        }

        self
    }
}

impl<const NUM_BYTES: usize> std::ops::Shl<usize> for SignedBignumFast<NUM_BYTES> {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        let shift = (rhs % 8) as u8;
        let mut bytes_shift = rhs / 8;

        if bytes_shift + self.len() > NUM_BYTES {
            bytes_shift = 0;
        }

        if bytes_shift > 0 {
            for i in (bytes_shift..self.len() + bytes_shift).rev() {
                self.digits[i] = self.digits[i - bytes_shift];
            }

            for i in 0..bytes_shift {
                self.digits[i] = 0;
            }
        }

        let mut carry = 0;
        for i in bytes_shift..self.len() + bytes_shift {
            let tmp_carry = (self.digits[i] as u16) >> (8 - shift);
            self.digits[i] <<= shift;
            self.digits[i] |= carry;
            carry = tmp_carry as u8;
        }

        self.pos += bytes_shift;
        if carry != 0 && self.len() < NUM_BYTES {
            self.digits[self.len()] = carry;
            self.pos += 1;
        }

        self
    }
}

impl<const NUM_BYTES: usize> std::ops::Add for SignedBignumFast<NUM_BYTES> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_ref(&rhs)
    }
}

impl<const NUM_BYTES: usize> std::ops::Sub for SignedBignumFast<NUM_BYTES> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_ref(&rhs)
    }
}

impl<const NUM_BYTES: usize> std::ops::Mul for SignedBignumFast<NUM_BYTES> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_ref(&rhs)
    }
}

impl<const NUM_BYTES: usize> From<i32> for SignedBignumFast<NUM_BYTES> {
    fn from(value: i32) -> Self {
        let mut bn = Self::from(value.unsigned_abs() as u128);
        bn.sign = value < 0;
        bn
    }
}

impl<const NUM_BYTES: usize> From<i128> for SignedBignumFast<NUM_BYTES> {
    fn from(value: i128) -> Self {
        let mut bn = Self::from(value.unsigned_abs());
        bn.sign = value < 0;
        bn
    }
}

impl<const NUM_BYTES: usize> From<u128> for SignedBignumFast<NUM_BYTES> {
    fn from(value: u128) -> Self {
        let mut bignum = SignedBignumFast::new();

        let mut pos_last_non_zero = 0;
        for i in 0..16 {
            let e = (value >> (i * 8)) as u8;
            if e != 0 {
                pos_last_non_zero = i;
            }
            bignum.digits[i] = e
        }
        bignum.pos = pos_last_non_zero;

        bignum
    }
}

impl<const NUM_BYTES: usize> From<UnsignedBignumFast<NUM_BYTES>> for SignedBignumFast<NUM_BYTES> {
    fn from(value: UnsignedBignumFast<NUM_BYTES>) -> Self {
        Self {
            digits: value.digits,
            pos: value.pos,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: usize = 20;

    fn check_pos<const NUM_BYTES: usize>(bn: &SignedBignumFast<NUM_BYTES>) {
        let mut pos_last_non_zero = 0;
        for (i, e) in bn.digits.iter().enumerate() {
            if *e != 0 {
                pos_last_non_zero = i;
            }
        }

        assert_eq!(pos_last_non_zero, bn.pos);
    }

    fn get_arithmatik_test_cases() -> Vec<(i128, i128)> {
        let mut test_cases: Vec<(i128, i128)> =
            vec![(0, 0), (0, 0xa), (0xa, 0), (0, -0xa), (-0xa, 0)];
        for a in (-0xabcedef..0xabcedef).step_by(5_000_000) {
            for b in (-0xabcedef..0xabcedef).step_by(5_000_000) {
                test_cases.push((a, b));
            }
        }

        test_cases
    }

    fn get_bit_manipulation_test_cases() -> Vec<(u128, usize)> {
        let base = 0xabcedef;
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..127 {
            test_cases.push((base, i));
        }

        test_cases
    }

    #[test]
    fn hex_string() {
        for (s, p) in [
            ("0x11abcdef", 3),
            ("0x1abcdef", 3),
            ("0xabcdef", 2),
            ("0x1cdef", 2),
            ("0xcdef", 1),
            ("0xdef", 1),
            ("0xef", 0),
            ("0xf", 0),
            ("0x0", 0),
            ("0x1", 0),
        ] {
            let bignum: SignedBignumFast<N> = SignedBignumFast::try_from_hex_string(s).unwrap();
            check_pos(&bignum);

            assert_eq!(p, bignum.pos);
            assert_eq!(s, bignum.to_hex_string());
        }
    }

    #[test]
    fn hex_string_2() {
        for (a, b) in get_arithmatik_test_cases() {
            let s = format!("{:#02x}", a.unsigned_abs() as u128);

            let bignum: SignedBignumFast<N> = SignedBignumFast::try_from_hex_string(&s).unwrap();
            check_pos(&bignum);

            let bignum2: SignedBignumFast<N> = SignedBignumFast::from(a);
            assert_eq!(bignum, bignum2);

            let s = format!("{:#02x}", b.unsigned_abs() as u128);

            let bignum: SignedBignumFast<N> = SignedBignumFast::try_from_hex_string(&s).unwrap();
            check_pos(&bignum);

            let bignum2: SignedBignumFast<N> = SignedBignumFast::from(b);
            assert_eq!(bignum, bignum2);
        }
    }

    #[test]
    fn len() {
        for (s, l) in [
            ("0x11abcdef", 4),
            ("0x1abcdef", 4),
            ("0xabcdef", 3),
            ("0x1cdef", 3),
            ("0xcdef", 2),
            ("0xdef", 2),
            ("0xef", 1),
            ("0xf", 1),
            ("0x0", 1),
            ("0x1", 1),
        ] {
            let bignum: SignedBignumFast<N> = SignedBignumFast::try_from_hex_string(s).unwrap();
            check_pos(&bignum);

            println!("{:?} {}", &bignum.digits[0..7], bignum.pos);
            assert_eq!(l, bignum.len());
        }
    }

    #[test]
    fn pos() {
        for (s, p) in [
            ("0x11abcdef", 3),
            ("0x1abcdef", 3),
            ("0xabcdef", 2),
            ("0x1cdef", 2),
            ("0xcdef", 1),
            ("0xdef", 1),
            ("0xef", 0),
            ("0xf", 0),
            ("0x0", 0),
            ("0x1", 0),
        ] {
            let bignum: SignedBignumFast<N> = SignedBignumFast::try_from_hex_string(s).unwrap();
            check_pos(&bignum);

            println!("{} {:?} {}", s, &bignum.digits[0..7], bignum.pos);
            assert_eq!(p, bignum.pos);
        }
    }

    #[test]
    fn from_little_endian() {
        for (e, s) in [
            (vec![0, 0], "0x0"),
            (vec![1, 0], "0x1"),
            (vec![0, 1, 0], "0x100"),
            (vec![0, 1, 2, 0], "0x20100"),
            (vec![0, 1, 2, 3, 0], "0x3020100"),
            (vec![0, 1, 2, 3, 4, 0], "0x403020100"),
        ] {
            let bignum: SignedBignumFast<N> = SignedBignumFast::from_little_endian(&e).unwrap();
            check_pos(&bignum);

            assert_eq!(bignum.to_hex_string(), s);
        }
    }

    #[test]
    fn from_big_endian() {
        for (e, s) in [
            (vec![0, 0], "0x0"),
            (vec![1, 0], "0x100"),
            (vec![0, 1, 0], "0x100"),
            (vec![0, 1, 2, 0], "0x10200"),
            (vec![0, 1, 2, 3, 0], "0x1020300"),
            (vec![0, 1, 2, 3, 4, 0], "0x102030400"),
        ] {
            let bignum: SignedBignumFast<N> = SignedBignumFast::from_big_endian(&e).unwrap();
            check_pos(&bignum);

            assert_eq!(bignum.to_hex_string(), s);
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
            let big_a: SignedBignumFast<N> = SignedBignumFast::from(a);
            check_pos(&big_a);

            let res: SignedBignumFast<N> = SignedBignumFast::from(a >> b);
            check_pos(&res);
            let res_big: SignedBignumFast<N> = big_a >> b;
            check_pos(&res_big);

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
            let big_a: SignedBignumFast<N> = SignedBignumFast::from(a);
            check_pos(&big_a);

            let res: SignedBignumFast<N> = SignedBignumFast::from(a << b);
            check_pos(&res);
            let res_big: SignedBignumFast<N> = big_a << b;
            check_pos(&res_big);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn addition() {
        for (a, b) in get_arithmatik_test_cases() {
            let big_a: SignedBignumFast<N> = SignedBignumFast::from(a);
            check_pos(&big_a);
            let big_b: SignedBignumFast<N> = SignedBignumFast::from(b);
            check_pos(&big_b);

            let res: SignedBignumFast<N> = SignedBignumFast::from(a + b);
            check_pos(&res);
            let res_big: SignedBignumFast<N> = big_a + big_b;
            check_pos(&res_big);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn subtraction() {
        for (a, b) in get_arithmatik_test_cases() {
            let big_a: SignedBignumFast<N> = SignedBignumFast::from(a);
            check_pos(&big_a);
            let big_b: SignedBignumFast<N> = SignedBignumFast::from(b);
            check_pos(&big_b);

            let res: SignedBignumFast<N> = SignedBignumFast::from(a - b);
            check_pos(&res);
            let res_big: SignedBignumFast<N> = big_a - big_b;
            check_pos(&res_big);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn multiplication() {
        for (a, b) in get_arithmatik_test_cases() {
            let big_a: SignedBignumFast<N> = SignedBignumFast::from(a);
            check_pos(&big_a);
            let big_b: SignedBignumFast<N> = SignedBignumFast::from(b);
            check_pos(&big_b);

            let res: SignedBignumFast<N> = SignedBignumFast::from(a * b);
            check_pos(&res);
            let res_big: SignedBignumFast<N> = big_a * big_b;
            check_pos(&res_big);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn division_with_remainder() {
        for (a, b) in get_arithmatik_test_cases() {
            if b == 0 {
                continue;
            }
            let big_a: SignedBignumFast<N> = SignedBignumFast::from(a);
            check_pos(&big_a);
            let big_b: SignedBignumFast<N> = SignedBignumFast::from(b);
            check_pos(&big_b);

            let (big_q, big_r) = SignedBignumFast::div_with_remainder(&big_a, &big_b);
            check_pos(&big_q);
            check_pos(&big_r);
            let q: SignedBignumFast<N> = SignedBignumFast::from(a / b);
            let r: SignedBignumFast<N> = SignedBignumFast::from(a % b);
            check_pos(&q);
            check_pos(&r);

            assert_eq!(q, big_q);
            assert_eq!(r, big_r);
        }
    }

    #[test]
    fn comparison() {
        for (a, b) in get_arithmatik_test_cases() {
            let big_a: SignedBignumFast<N> = SignedBignumFast::from(a);
            let big_b: SignedBignumFast<N> = SignedBignumFast::from(b);
            check_pos(&big_a);
            check_pos(&big_b);

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
    fn get_bit() {
        for (a, b) in get_bit_manipulation_test_cases() {
            let big_a: SignedBignumFast<N> = SignedBignumFast::from(a);
            check_pos(&big_a);

            let res = (a >> b) & 1 == 1;
            let res_big = big_a.get_bit(b);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn set_bit() {
        for (mut a, b) in get_bit_manipulation_test_cases() {
            let mut big_a: SignedBignumFast<N> = SignedBignumFast::from(a);
            check_pos(&big_a);
            big_a.set_bit(b);
            check_pos(&big_a);

            a |= 1 << b;
            let a = SignedBignumFast::from(a);
            check_pos(&a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn unset_bit() {
        for (mut a, b) in get_bit_manipulation_test_cases() {
            let mut big_a: SignedBignumFast<N> = SignedBignumFast::from(a);
            check_pos(&big_a);
            big_a.unset_bit(b);
            check_pos(&big_a);

            a &= !(1 << b);
            let a = SignedBignumFast::from(a);
            check_pos(&a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn toggle_bit() {
        for (mut a, b) in get_bit_manipulation_test_cases() {
            let mut big_a: SignedBignumFast<N> = SignedBignumFast::from(a);
            check_pos(&big_a);
            big_a.toggle_bit(b);
            check_pos(&big_a);

            a ^= 1 << b;
            let a = SignedBignumFast::from(a);
            check_pos(&a);

            assert_eq!(a, big_a);
        }
    }
}
