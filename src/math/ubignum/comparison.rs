use super::bignum::UBignum;

impl<const N: usize> PartialEq for UBignum<N> {
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

impl<const NUM_BYTES: usize> PartialOrd for UBignum<NUM_BYTES> {
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

#[cfg(test)]
mod tests {
    use crate::math::ubignum::utils::{check_pos, get_arithmatik_test_cases};

    use super::*;
    const N: usize = 3;

    #[test]
    fn comparison() {
        for (a, b) in get_arithmatik_test_cases() {
            let big_a: UBignum<N> = UBignum::from(a);
            let big_b: UBignum<N> = UBignum::from(b);
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
}
