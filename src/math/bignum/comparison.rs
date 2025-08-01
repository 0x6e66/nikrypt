use crate::math::bignum::Bignum;

impl PartialEq for Bignum {
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

impl PartialOrd for Bignum {
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

#[cfg(test)]
mod test {
    use crate::math::bignum::{get_test_cases, Bignum};

    #[test]
    fn comparison() {
        for (a, b) in get_test_cases() {
            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

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
