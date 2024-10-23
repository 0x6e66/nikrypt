/// Internal storage in little endian
///
/// 0xabcdef00 -> Bignum([0x00, 0xef, 0xcd, 0xab])
#[derive(Debug)]
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

        if let Some(tmp) = res.strip_prefix("0") {
            res = tmp.to_string();
        }

        format!("0x{}", res)
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

        if self.0.len() == 0 {
            self.0.push(0u8);
        }
    }
}

impl From<u128> for Bignum {
    fn from(value: u128) -> Self {
        let mut res = Self(vec![
            value as u8,
            (value >> 1 * 8) as u8,
            (value >> 2 * 8) as u8,
            (value >> 3 * 8) as u8,
            (value >> 4 * 8) as u8,
            (value >> 5 * 8) as u8,
            (value >> 6 * 8) as u8,
            (value >> 7 * 8) as u8,
            (value >> 8 * 8) as u8,
            (value >> 9 * 8) as u8,
            (value >> 10 * 8) as u8,
            (value >> 11 * 8) as u8,
            (value >> 12 * 8) as u8,
            (value >> 13 * 8) as u8,
            (value >> 14 * 8) as u8,
            (value >> 15 * 8) as u8,
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

        return true;
    }
}

impl PartialOrd for Bignum {
    fn lt(&self, other: &Self) -> bool {
        if self.0.len() > other.0.len() {
            return false;
        } else if self.0.len() < other.0.len() {
            return true;
        }

        for (s, o) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if s > o {
                return false;
            } else if s < o {
                return true;
            }
        }

        return false;
    }

    fn le(&self, other: &Self) -> bool {
        if self.0.len() > other.0.len() {
            return false;
        } else if self.0.len() < other.0.len() {
            return true;
        }

        for (s, o) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if s > o {
                return false;
            } else if s < o {
                return true;
            }
        }

        return true;
    }

    fn gt(&self, other: &Self) -> bool {
        if self.0.len() > other.0.len() {
            return true;
        } else if self.0.len() < other.0.len() {
            return false;
        }

        for (s, o) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if s > o {
                return true;
            } else if s < o {
                return false;
            }
        }

        return false;
    }

    fn ge(&self, other: &Self) -> bool {
        if self.0.len() > other.0.len() {
            return true;
        } else if self.0.len() < other.0.len() {
            return false;
        }

        for (s, o) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if s > o {
                return true;
            } else if s < o {
                return false;
            }
        }

        return true;
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.0.len() > other.0.len() {
            return Some(std::cmp::Ordering::Greater);
        } else if self.0.len() < other.0.len() {
            return Some(std::cmp::Ordering::Less);
        }

        for (s, o) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if s > o {
                return Some(std::cmp::Ordering::Greater);
            } else if s < o {
                return Some(std::cmp::Ordering::Less);
            }
        }

        return Some(std::cmp::Ordering::Equal);
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
            let tmp_carry = (*b << (8 - shift)) as u8;
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
            let tmp_carry = (*b >> (8 - shift)) as u8;
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
        let (long, mut short) = match self.0.len() > rhs.0.len() {
            true => (self, rhs),
            false => (rhs, self),
        };

        short.0.resize(long.0.len(), 0u8);

        let mut carry = 0;
        for i in 0..long.0.len() {
            let tmp = short.0[i] as u32 + long.0[i] as u32 + carry;
            carry = tmp >> 8;

            short.0[i] = tmp as u8;
        }

        if carry != 0 {
            short.0.push(carry as u8);
        }

        short
    }
}

impl std::ops::Mul for Bignum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (p, a) = (self.0.len(), self.0);
        let (q, b) = (rhs.0.len(), rhs.0);
        let base = 256;

        let mut product = Vec::new();
        product.resize(p + q, 0u8);

        for b_i in 0..q {
            let mut carry = 0;
            for a_i in 0..p {
                let mut tmp = product[a_i + b_i] as u32;
                tmp += carry + a[a_i] as u32 * b[b_i] as u32;
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
}
