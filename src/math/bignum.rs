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
        if self.0.len() == 1 {
            return String::from("0x00");
        }

        let mut res = String::from("0x");
        let mut leading_zeros = true;

        for b in self.0.iter().rev() {
            if *b == 0 && leading_zeros {
                continue;
            } else if *b != 0 {
                leading_zeros = false;
            }

            res.push_str(&format!("{:02x}", b));
        }

        res
    }

    pub fn num_of_bytes(&self) -> usize {
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
            let (mut sum, mut tmp_carry) = long.0[i].overflowing_add(carry);
            carry = 0;
            if tmp_carry {
                carry += 1;
            }

            (sum, tmp_carry) = sum.overflowing_add(short.0[i]);
            if tmp_carry {
                carry += 1;
            }

            short.0[i] = sum;
        }

        if carry != 0 {
            short.0.push(carry);
        }

        short
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_1() {
        let a = 0xaabb0000;
        let b = 0x0000ccdd;

        let big_a = Bignum::from(a);
        let big_b = Bignum::from(b);

        let res = Bignum::from(a + b);
        let res_big = big_a + big_b;

        assert_eq!(res, res_big);
    }

    #[test]
    fn add_2() {
        let a = 0xffff;
        let b = 0xffff;

        let big_a = Bignum::from(a);
        let big_b = Bignum::from(b);

        let res = Bignum::from(a + b);
        let res_big = big_a + big_b;

        assert_eq!(res, res_big);
    }
}
