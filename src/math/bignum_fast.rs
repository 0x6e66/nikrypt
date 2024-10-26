const NUM_BYTES: usize = 5000;

#[derive(Debug, Clone)]
pub struct BignumFast {
    digits: [u8; NUM_BYTES],
    pos: usize,
}

fn check_byte_length(value: &[u8]) -> bool {
    value.len() > NUM_BYTES
}

fn calc_pos(length: usize) -> usize {
    if length <= 2 {
        return 0;
    } else if length % 2 == 0 {
        return length / 2 - 1;
    } else {
        return length / 2;
    }
}

impl BignumFast {
    pub fn new() -> Self {
        BignumFast::zero()
    }

    pub fn zero() -> Self {
        Self {
            digits: [0; NUM_BYTES],
            pos: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.pos + 1
    }

    pub fn from_big_endian(value: &[u8]) -> Option<Self> {
        if check_byte_length(value) {
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
        if check_byte_length(value) {
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
        let s = s.trim_start_matches("0");

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

    pub fn add_ref(&self, rhs: &Self) -> Self {
        let (long, short) = match self.pos > rhs.pos {
            true => (self, rhs),
            false => (rhs, self),
        };

        let mut bignum = BignumFast::new();
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

        let mut bignum = BignumFast::new();

        let mut pos_last_non_zero = 0;
        for b_i in 0..q {
            let mut carry = 0;
            for a_i in 0..p {
                let mut tmp = bignum.digits[a_i + b_i] as u16;
                tmp += carry + self.digits[a_i] as u16 * other.digits[b_i] as u16;
                carry = tmp / base;
                tmp %= base;
                bignum.digits[a_i + b_i] = tmp as u8;
            }
            if carry != 0 {
                pos_last_non_zero = b_i + p;
            }
            bignum.digits[b_i + p] = carry as u8;
        }

        bignum.pos = pos_last_non_zero;

        bignum
    }
}

impl PartialEq for BignumFast {
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

impl PartialOrd for BignumFast {
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

impl std::ops::Shr<usize> for BignumFast {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        let shift = (rhs % 8) as u8;
        let bytes_shift = rhs / 8;

        if bytes_shift >= self.len() {
            return Self::zero();
        }

        for i in 0..self.len() - bytes_shift + 1 {
            self.digits[i] = self.digits[i + bytes_shift];
        }

        let mut carry = 0;
        for i in (0..self.len()).rev() {
            let tmp_carry = self.digits[i] << (8 - shift);
            self.digits[i] >>= shift;
            self.digits[i] |= carry;
            carry = tmp_carry;
        }

        self.pos -= bytes_shift;
        if self.digits[self.pos] == 0 && self.pos > 0 {
            self.pos -= 1;
        }

        self
    }
}

impl std::ops::Shl<usize> for BignumFast {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        let shift = (rhs % 8) as u8;
        let mut bytes_shift = rhs / 8;

        if bytes_shift + self.len() > NUM_BYTES {
            bytes_shift = 0;
        }

        if bytes_shift > 0 {
            for i in (bytes_shift..self.len() + bytes_shift).rev() {
                self.digits[i] = self.digits[i - bytes_shift as usize];
            }

            for i in 0..bytes_shift {
                self.digits[i] = 0;
            }
        }

        let mut carry = 0;
        for i in bytes_shift..self.len() + bytes_shift {
            let tmp_carry = self.digits[i] >> (8 - shift);
            self.digits[i] <<= shift;
            self.digits[i] |= carry;
            carry = tmp_carry;
        }

        self.pos += bytes_shift;
        if carry != 0 && self.len() < NUM_BYTES {
            self.digits[self.len()] = carry;
            self.pos += 1;
        }

        self
    }
}

impl std::ops::Add for BignumFast {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_ref(&rhs)
    }
}

impl std::ops::Sub for BignumFast {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_ref(&rhs)
    }
}

impl std::ops::Mul for BignumFast {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_ref(&rhs)
    }
}

impl From<u128> for BignumFast {
    fn from(value: u128) -> Self {
        let mut bignum = BignumFast::new();

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

#[cfg(test)]
mod tests {
    use super::*;

    fn check_pos(bn: &BignumFast) {
        let mut pos_last_non_zero = 0;
        for (i, e) in bn.digits.iter().enumerate() {
            if *e != 0 {
                pos_last_non_zero = i;
            }
        }

        assert_eq!(bn.pos, pos_last_non_zero);
    }

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
            let bignum = BignumFast::try_from_hex_string(s).unwrap();
            check_pos(&bignum);

            println!("{} {:?} {}", s, &bignum.digits[0..7], bignum.pos);
            assert_eq!(p, bignum.pos);
            assert_eq!(s, bignum.to_hex_string());
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
            let bignum = BignumFast::try_from_hex_string(s).unwrap();
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
            let bignum = BignumFast::try_from_hex_string(s).unwrap();
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
            let bignum = BignumFast::from_little_endian(&e).unwrap();
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
            let bignum = BignumFast::from_big_endian(&e).unwrap();
            check_pos(&bignum);

            assert_eq!(bignum.to_hex_string(), s);
        }
    }

    #[test]
    fn from_u128() {
        for (a, b) in NUM_PAIRS {
            let bignum = BignumFast::from(a);
            check_pos(&bignum);
            let s = format!("{:#02x}", a);
            assert_eq!(bignum.to_hex_string(), s);

            let bignum = BignumFast::from(b);
            check_pos(&bignum);
            let s = format!("{:#02x}", b);
            assert_eq!(bignum.to_hex_string(), s);
        }
    }

    #[test]
    fn shift_right() {
        for (a, b) in NUM_PAIRS2 {
            let big_a = BignumFast::from(a);
            check_pos(&big_a);

            let (tmp, _) = a.overflowing_shr(b as u32);
            let res = BignumFast::from(tmp);
            check_pos(&res);
            let res_big = big_a >> b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn shift_left() {
        for (a, b) in NUM_PAIRS2 {
            let big_a = BignumFast::from(a);
            check_pos(&big_a);

            let tmp = a << b as u32;
            let res = BignumFast::from(tmp);
            check_pos(&res);
            let res_big = big_a << b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn addition() {
        for (a, b) in NUM_PAIRS {
            let big_a = BignumFast::from(a);
            check_pos(&big_a);
            let big_b = BignumFast::from(b);
            check_pos(&big_b);

            let res = BignumFast::from(a + b);
            check_pos(&res);
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

            let big_a = BignumFast::from(a);
            check_pos(&big_a);
            let big_b = BignumFast::from(b);
            check_pos(&big_b);

            let res = BignumFast::from(a - b);
            check_pos(&res);
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

            let big_a = BignumFast::from(a);
            check_pos(&big_a);
            let big_b = BignumFast::from(b);
            check_pos(&big_b);

            let _res_big = big_a - big_b;
        }
    }

    #[test]
    fn multiplication() {
        for (a, b) in NUM_PAIRS {
            let big_a = BignumFast::from(a);
            check_pos(&big_a);
            let big_b = BignumFast::from(b);
            check_pos(&big_b);

            let res = BignumFast::from(a * b);
            check_pos(&res);
            let res_big = big_a * big_b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn comparison() {
        for (a, b) in NUM_PAIRS {
            let big_a = BignumFast::from(a);
            check_pos(&big_a);
            let big_b = BignumFast::from(b);
            check_pos(&big_b);

            let res = a.partial_cmp(&b);
            let res_big = big_a.partial_cmp(&big_b);

            assert_eq!(res, res_big);
            println!();
        }
    }

    #[test]
    fn get_bit() {
        for (a, b) in NUM_PAIRS2 {
            let big_a = BignumFast::from(a);
            check_pos(&big_a);

            let res = (a >> b) & 1 == 1;
            let res_big = big_a.get_bit(b);

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn set_bit() {
        for (mut a, b) in NUM_PAIRS2 {
            let mut big_a = BignumFast::from(a);
            check_pos(&big_a);
            big_a.set_bit(b);

            a |= 1 << b;
            let a = BignumFast::from(a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn unset_bit() {
        for (mut a, b) in NUM_PAIRS2 {
            let mut big_a = BignumFast::from(a);
            check_pos(&big_a);
            big_a.unset_bit(b);

            a &= !(1 << b);
            let a = BignumFast::from(a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn toggle_bit() {
        for (mut a, b) in NUM_PAIRS2 {
            let mut big_a = BignumFast::from(a);
            check_pos(&big_a);
            big_a.toggle_bit(b);

            a ^= 1 << b;
            let a = BignumFast::from(a);

            assert_eq!(a, big_a);
        }
    }
}
