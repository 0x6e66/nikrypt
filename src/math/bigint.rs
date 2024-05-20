#[derive(Debug, PartialEq)]
pub struct BigInt(Vec<u8>);

impl From<&[u8]> for BigInt {
    fn from(value: &[u8]) -> Self {
        Self(Vec::from(value))
    }
}

impl From<Vec<u8>> for BigInt {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<u8> for BigInt {
    fn from(value: u8) -> Self {
        Self(vec![value])
    }
}

impl From<u128> for BigInt {
    fn from(value: u128) -> Self {
        let mut tmp = Self(vec![
            value as u8,
            (value >> (1 * 8)) as u8,
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
        tmp.trim();
        tmp
    }
}

impl std::ops::Index<usize> for BigInt {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.get(index).unwrap()
    }
}

impl std::ops::IndexMut<usize> for BigInt {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.get_mut(index).unwrap()
    }
}

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str_buf = String::new();
        for b in self.0.iter().rev() {
            str_buf.push_str(&format!("{:02x} ", b));
        }
        write!(f, "{}", str_buf)
    }
}

impl std::ops::Add for BigInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let self_len = self.len();
        let other_len = rhs.len();

        let (mut long, mut short, len) = match self_len > other_len {
            true => (self, rhs, self_len),
            false => (rhs, self, other_len),
        };

        short.0.resize(len, 0);

        let mut carry = 0u8;

        for i in 0..len {     
            let mut new_carry = 0;       
            let (tmp, overflow) = long[i].overflowing_add(short[i]);
            if overflow {
                new_carry += 1;
            }

            let (tmp, overflow) = tmp.overflowing_add(carry);
            if overflow {
                new_carry += 1;
            }

            long[i] = tmp;
            carry = new_carry;
        }

        if carry > 0 {
            long.0.push(carry);
        }

        long
    }
}

impl BigInt {
    fn new() -> Self {
        Self(vec![0])
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn trim(&mut self) {
        let mut count = 0;
        let len= self.len();
        for i in (0..len).rev() {
            if self[i] != 0 {
                break;
            }
            count += 1;
        }
        self.0.resize(len - count, 0);
    }

    fn from_big_endian(&self, mut vec: Vec<u8>) -> Self{
        vec.reverse();
        Self::from(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition_1() {
        let (a,b) = (15u8, 30u8);

        let a_big = BigInt::from(a);
        let b_big = BigInt::from(b);

        let c = a_big + b_big;
        assert_eq!(c, BigInt::from(a+b));
    }

    #[test]
    fn test_addition_2() {
        let (a,b) = (0xffffu128, 0xffffu128);

        let a_big = BigInt::from(a);
        let b_big = BigInt::from(b);

        let c = a_big + b_big;
        assert_eq!(c, BigInt::from(a+b));
    }

    #[test]
    fn test_addition_3() {
        let (a,b) = (0xffu128, 0x1u128);

        let a_big = BigInt::from(a);
        let b_big = BigInt::from(b);

        let c = a_big + b_big;
        assert_eq!(c, BigInt::from(a+b));
    }
}
