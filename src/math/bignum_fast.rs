#[derive(Debug, Clone)]
pub struct BignumFast<const NUM_DIGITS: usize> {
    digits: [u64; NUM_DIGITS],
    pos: usize,
}

fn calc_pos_from_num_of_bytes(length: usize) -> usize {
    if length <= 16 {
        0
    } else if length % 2 == 0 {
        length / 16 - 1
    } else {
        length / 16
    }
}

impl<const NUM_DIGITS: usize> BignumFast<NUM_DIGITS> {
    pub fn new() -> Self {
        BignumFast::zero()
    }

    pub fn zero() -> Self {
        Self {
            digits: [0; NUM_DIGITS],
            pos: 0,
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
        if value.len() > NUM_DIGITS * 8 {
            return None;
        }

        let mut bignum = Self::new();

        let mut pos_last_non_zero = 0;
        for (i, e) in value.chunks(8).rev().enumerate() {
            let num = u64::from_be_bytes(e.try_into().ok()?);
            if num != 0 {
                pos_last_non_zero = i;
            }
            bignum.digits[i] = num;
        }

        bignum.pos = calc_pos_from_num_of_bytes(value.len() * 2);
        if bignum.pos > 0 {
            bignum.pos = pos_last_non_zero;
        }

        Some(bignum)
    }

    pub fn from_little_endian(value: &[u8]) -> Option<Self> {
        if value.len() > NUM_DIGITS * 8 {
            return None;
        }

        let mut bignum = Self::new();

        let mut pos_last_non_zero = 0;
        for (i, e) in value.chunks(8).enumerate() {
            let num = u64::from_le_bytes(e.try_into().ok()?);
            if num != 0 {
                pos_last_non_zero = i;
            }
            bignum.digits[i] = num;
        }

        bignum.pos = calc_pos_from_num_of_bytes(value.len() * 2);
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

        bignum.pos = calc_pos_from_num_of_bytes(len);

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

            res.push_str(&format!("{:02x}", b));
        }

        if let Some(tmp) = res.strip_prefix('0') {
            res = tmp.to_string();
        }

        format!("0x{}", res)
    }

    pub fn get_bit(&self, pos: usize) -> bool {
        let digit_pos = pos / 64;
        if pos >= NUM_DIGITS * 64 {
            panic!("Bit index out of bounds. Max index is {}", NUM_DIGITS * 64);
        }

        let byte = self.digits[digit_pos];
        (byte >> (pos % 64)) & 1 == 1
    }

    pub fn set_bit(&mut self, pos: usize) {
        let byte_pos = pos / 64;
        if byte_pos >= NUM_DIGITS {
            panic!("Bit index out of bounds. Max index is {}", NUM_DIGITS * 64);
        }

        self.digits[byte_pos] |= 1 << (pos % 64);

        if byte_pos > self.pos {
            self.pos = byte_pos;
        }
    }

    pub fn unset_bit(&mut self, pos: usize) {
        let byte_pos = pos / 64;
        if byte_pos >= NUM_DIGITS {
            panic!("Bit index out of bounds. Max index is {}", NUM_DIGITS * 64);
        }

        self.digits[byte_pos] &= !(1 << (pos % 64));

        for (i, e) in self.digits[0..self.len()].iter().enumerate().rev() {
            if *e != 0 || i == 0 {
                self.pos = i;
                return;
            }
        }
    }

    pub fn toggle_bit(&mut self, pos: usize) {
        let byte_pos = pos / 64;
        if byte_pos >= NUM_DIGITS {
            panic!("Bit index out of bounds. Max index is {}", NUM_DIGITS * 64);
        }

        self.digits[byte_pos] ^= 1 << (pos % 64);

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

    pub fn add_ref(&self, rhs: &Self) -> Self {
        let (long, short) = match self.pos > rhs.pos {
            true => (self, rhs),
            false => (rhs, self),
        };

        let mut bignum = BignumFast::new();
        bignum.pos = long.pos;

        let mut carry = 0;
        for i in 0..long.len() {
            let mut tmp = long.digits[i] as u128 + carry;
            if i < short.len() {
                tmp += short.digits[i] as u128;
            }
            carry = tmp >> 64;
            bignum.digits[i] = tmp as u64;
        }

        if carry != 0 {
            if bignum.len() == NUM_DIGITS {
                panic!("Attempted addition with overflow");
            }
            bignum.digits[bignum.len()] = carry as u64;
            bignum.pos += 1;
        }

        bignum
    }

    pub fn sub_ref(&self, rhs: &Self) -> Self {
        match self.partial_cmp(rhs) {
            Some(std::cmp::Ordering::Less) => panic!(
                "Result of subtraction would be negative.\nlhs: {}\nrhs: {}",
                self.to_hex_string(),
                rhs.to_hex_string()
            ),
            Some(std::cmp::Ordering::Equal) => return BignumFast::zero(),
            _ => (),
        }

        let (long, short) = (self, rhs);
        let mut bignum = BignumFast::new();

        let mut carry = 0;
        let mut pos_last_non_zero = 0;
        for i in 0..long.len() {
            let (mut sum, mut tmp_carry) = long.digits[i].overflowing_sub(carry);
            carry = tmp_carry as u64;

            if i < short.len() {
                (sum, tmp_carry) = sum.overflowing_sub(short.digits[i]);
                carry += tmp_carry as u64;
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
        // let base = 256;
        let base = u64::MAX as u128 + 1;

        if p + q > NUM_DIGITS {
            panic!("Attempted multiplication with overflow");
        }

        let mut bignum = BignumFast::new();

        let mut pos_last_non_zero = 0;
        for b_i in 0..q {
            let mut carry = 0;
            for a_i in 0..p {
                let mut tmp = bignum.digits[a_i + b_i] as u128;
                tmp += carry + self.digits[a_i] as u128 * other.digits[b_i] as u128;
                carry = tmp / base;
                tmp %= base;
                bignum.digits[a_i + b_i] = tmp as u64;
                if tmp != 0 {
                    pos_last_non_zero = a_i + b_i;
                }
            }
            if carry != 0 {
                pos_last_non_zero = b_i + p;
            }
            bignum.digits[b_i + p] = carry as u64;
        }

        bignum.pos = pos_last_non_zero;

        bignum
    }

    pub fn div_with_remainder(&self, rhs: &Self) -> (Self, Self) {
        let mut q = BignumFast::new();
        let mut r = BignumFast::new();

        let (n_len, n) = (self.len() * 64, self);

        for i in (0..n_len).rev() {
            r = r << 1;
            if n.get_bit(i) {
                r.set_bit(0);
            } else {
                r.unset_bit(0);
            }

            if r >= *rhs {
                r = r.sub_ref(rhs);
                q.set_bit(i);
            }
        }

        (q, r)
    }

    pub fn pow_mod(self, exponent: Self, modulus: &Self) -> Self {
        let mut base = self;
        let mut exp = exponent;

        let mut t = BignumFast::from(1);
        while !exp.is_zero() {
            if !exp.is_even() {
                (_, t) = BignumFast::mul_ref(&t, &base).div_with_remainder(modulus);
            }
            (_, base) = BignumFast::mul_ref(&base, &base).div_with_remainder(modulus);
            exp = exp >> 1;
        }
        let (_, r) = t.div_with_remainder(modulus);
        r
    }
}

impl<const NUM_DIGITS: usize> Default for BignumFast<NUM_DIGITS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const NUM_DIGITS: usize> PartialEq for BignumFast<NUM_DIGITS> {
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

impl<const NUM_DIGITS: usize> PartialOrd for BignumFast<NUM_DIGITS> {
    fn lt(&self, other: &Self) -> bool {
        if self.pos != other.pos {
            return self.pos.lt(&other.pos);
        }

        for (s, o) in self.digits[0..self.len()]
            .iter()
            .rev()
            .zip(other.digits[0..self.len()].iter().rev())
        {
            if s != o {
                return s.lt(o);
            }
        }

        false
    }

    fn le(&self, other: &Self) -> bool {
        if self.pos != other.pos {
            return self.pos.lt(&other.pos);
        }

        for (s, o) in self.digits[0..self.len()]
            .iter()
            .rev()
            .zip(other.digits[0..self.len()].iter().rev())
        {
            if s != o {
                return s.lt(o);
            }
        }

        true
    }

    fn gt(&self, other: &Self) -> bool {
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

    fn ge(&self, other: &Self) -> bool {
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

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.pos != other.pos {
            return Some(self.pos.cmp(&other.pos));
        }

        for (s, o) in self.digits[0..self.len()]
            .iter()
            .rev()
            .zip(other.digits[0..self.len()].iter().rev())
        {
            if s != o {
                return Some(s.cmp(o));
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl<const NUM_DIGITS: usize> std::ops::Shr<usize> for BignumFast<NUM_DIGITS> {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        let shift = (rhs % 64) as u64;
        let digits_shift = rhs / 64;

        if digits_shift >= self.len() {
            return Self::zero();
        }

        for i in 0..self.len() {
            self.digits[i] = self.digits[i + digits_shift];
        }

        if shift == 0 {
            self.pos -= digits_shift;
            return self;
        }

        let mut carry = 0;
        for i in (0..self.len()).rev() {
            let tmp_carry = (self.digits[i] as u128) << (64 - shift);
            self.digits[i] >>= shift;
            self.digits[i] |= carry;
            carry = tmp_carry as u64;
        }

        self.pos -= digits_shift;
        if self.digits[self.pos] == 0 && self.pos > 0 {
            self.pos -= 1;
        }

        self
    }
}

impl<const NUM_DIGITS: usize> std::ops::Shl<usize> for BignumFast<NUM_DIGITS> {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        let shift = (rhs % 64) as u64;
        let mut digits_shift = rhs / 64;

        if digits_shift + self.len() > NUM_DIGITS {
            digits_shift = 0;
        }

        if digits_shift > 0 {
            for i in (digits_shift..self.len() + digits_shift).rev() {
                self.digits[i] = self.digits[i - digits_shift];
            }

            for i in 0..digits_shift {
                self.digits[i] = 0;
            }
        }

        let mut carry = 0;
        for i in digits_shift..self.len() + digits_shift {
            let tmp_carry = (self.digits[i] as u128) >> (64 - shift);
            self.digits[i] <<= shift;
            self.digits[i] |= carry;
            carry = tmp_carry as u64;
        }

        self.pos += digits_shift;
        if carry != 0 && self.len() < NUM_DIGITS {
            self.digits[self.len()] = carry;
            self.pos += 1;
        }

        self
    }
}

impl<const NUM_DIGITS: usize> std::ops::Add for BignumFast<NUM_DIGITS> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_ref(&rhs)
    }
}

impl<const NUM_DIGITS: usize> std::ops::Sub for BignumFast<NUM_DIGITS> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_ref(&rhs)
    }
}

impl<const NUM_DIGITS: usize> std::ops::Mul for BignumFast<NUM_DIGITS> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_ref(&rhs)
    }
}

impl<const NUM_DIGITS: usize> From<u128> for BignumFast<NUM_DIGITS> {
    fn from(value: u128) -> Self {
        let mut bignum = BignumFast::new();

        let high = (value >> 64) as u64;
        let low = value as u64;

        bignum.digits[0] = low;
        bignum.digits[1] = high;

        if high != 0 {
            bignum.pos = 1;
        }

        bignum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: usize = 64;

    fn check_pos<const NUM_DIGITS: usize>(bn: &BignumFast<NUM_DIGITS>) {
        let mut pos_last_non_zero = 0;
        for (i, e) in bn.digits.iter().enumerate() {
            if *e != 0 {
                pos_last_non_zero = i;
            }
        }

        assert_eq!(pos_last_non_zero, bn.pos);
    }

    fn get_arithmatik_test_cases() -> Vec<(u128, u128)> {
        let mut test_cases: Vec<(u128, u128)> = vec![(0, 0xa), (0xa, 0), (0, 0)];
        for a in (0..0xabcedef).step_by(5_000_000) {
            for b in (0..0xabcedef).step_by(5_000_000) {
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
            let bignum: BignumFast<N> = BignumFast::try_from_hex_string(s).unwrap();
            check_pos(&bignum);

            assert_eq!(s, bignum.to_hex_string());
        }
    }

    #[test]
    fn hex_string_2() {
        for (a, b) in get_arithmatik_test_cases() {
            let s = format!("{:#02x}", a);

            let bignum: BignumFast<N> = BignumFast::try_from_hex_string(&s).unwrap();
            check_pos(&bignum);

            let bignum2: BignumFast<N> = BignumFast::from(a);
            assert_eq!(bignum, bignum2);

            let s = format!("{:#02x}", b);

            let bignum: BignumFast<N> = BignumFast::try_from_hex_string(&s).unwrap();
            check_pos(&bignum);

            let bignum2: BignumFast<N> = BignumFast::from(b);
            assert_eq!(bignum, bignum2);
        }
    }

    #[test]
    fn from_u128() {
        for (a, b) in get_arithmatik_test_cases() {
            let bignum: BignumFast<N> = BignumFast::from(a);
            check_pos(&bignum);
            let s = format!("{:#02x}", a);
            assert_eq!(bignum.to_hex_string(), s);

            let bignum: BignumFast<N> = BignumFast::from(b);
            check_pos(&bignum);
            let s = format!("{:#02x}", b);
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
            let big_a: BignumFast<N> = BignumFast::from(a);
            check_pos(&big_a);

            let res: BignumFast<N> = BignumFast::from(a >> b);
            check_pos(&res);
            let res_big: BignumFast<N> = big_a >> b;
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
            let big_a: BignumFast<N> = BignumFast::from(a);
            check_pos(&big_a);

            let res: BignumFast<N> = BignumFast::from(a << b);
            check_pos(&res);
            let res_big: BignumFast<N> = big_a << b;
            check_pos(&res_big);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn addition() {
        for (a, b) in get_arithmatik_test_cases() {
            let big_a: BignumFast<N> = BignumFast::from(a);
            check_pos(&big_a);
            let big_b: BignumFast<N> = BignumFast::from(b);
            check_pos(&big_b);

            let res: BignumFast<N> = BignumFast::from(a + b);
            check_pos(&res);
            let res_big: BignumFast<N> = big_a + big_b;
            check_pos(&res_big);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn subtraction() {
        for (a, b) in get_arithmatik_test_cases() {
            let (a, b) = match a >= b {
                true => (a, b),
                false => (b, a),
            };

            let big_a: BignumFast<N> = BignumFast::from(a);
            check_pos(&big_a);
            let big_b: BignumFast<N> = BignumFast::from(b);
            check_pos(&big_b);

            let res: BignumFast<N> = BignumFast::from(a - b);
            check_pos(&res);
            let res_big: BignumFast<N> = big_a - big_b;
            check_pos(&res_big);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    #[should_panic]
    fn subtraction_panic() {
        for (a, b) in get_arithmatik_test_cases() {
            let (a, b) = match a >= b {
                false => (a, b),
                true => (b, a),
            };

            let big_a: BignumFast<N> = BignumFast::from(a);
            check_pos(&big_a);
            let big_b: BignumFast<N> = BignumFast::from(b);
            check_pos(&big_b);

            let _res_big: BignumFast<N> = big_a - big_b;
        }
    }

    #[test]
    fn multiplication() {
        for (a, b) in get_arithmatik_test_cases() {
            let big_a: BignumFast<N> = BignumFast::from(a);
            check_pos(&big_a);
            let big_b: BignumFast<N> = BignumFast::from(b);
            check_pos(&big_b);

            let res: BignumFast<N> = BignumFast::from(a * b);
            check_pos(&res);
            let res_big: BignumFast<N> = big_a * big_b;
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
            let big_a: BignumFast<N> = BignumFast::from(a);
            check_pos(&big_a);
            let big_b: BignumFast<N> = BignumFast::from(b as u128);
            check_pos(&big_b);

            let (big_q, big_r) = BignumFast::div_with_remainder(&big_a, &big_b);
            check_pos(&big_q);
            check_pos(&big_r);
            let q: BignumFast<N> = BignumFast::from(a / b);
            let r: BignumFast<N> = BignumFast::from(a % b);
            check_pos(&q);
            check_pos(&r);

            assert_eq!(big_q, q);
            assert_eq!(big_r, r);
        }
    }

    #[test]
    fn comparison() {
        for (a, b) in get_arithmatik_test_cases() {
            let big_a: BignumFast<N> = BignumFast::from(a);
            let big_b: BignumFast<N> = BignumFast::from(b);
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
            let big_a: BignumFast<N> = BignumFast::from(a);
            check_pos(&big_a);

            let res = (a >> b) & 1 == 1;
            let res_big = big_a.get_bit(b);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn set_bit() {
        for (mut a, b) in get_bit_manipulation_test_cases() {
            let mut big_a: BignumFast<N> = BignumFast::from(a);
            check_pos(&big_a);
            big_a.set_bit(b);
            check_pos(&big_a);

            a |= 1 << b;
            let a = BignumFast::from(a);
            check_pos(&a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn unset_bit() {
        for (mut a, b) in get_bit_manipulation_test_cases() {
            let mut big_a: BignumFast<N> = BignumFast::from(a);
            check_pos(&big_a);
            big_a.unset_bit(b);
            check_pos(&big_a);

            a &= !(1 << b);
            let a = BignumFast::from(a);
            check_pos(&a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn toggle_bit() {
        for (mut a, b) in get_bit_manipulation_test_cases() {
            let mut big_a: BignumFast<N> = BignumFast::from(a);
            check_pos(&big_a);
            big_a.toggle_bit(b);
            check_pos(&big_a);

            a ^= 1 << b;
            let a = BignumFast::from(a);
            check_pos(&a);

            assert_eq!(a, big_a);
        }
    }
}
