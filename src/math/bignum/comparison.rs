use std::cmp::Ordering;

use crate::math::bignum::Bignum;

impl PartialEq for Bignum {
    // TODO: maybe derive Eq on Bignum
    fn eq(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return false;
        }

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

impl Bignum {
    // treats both BNs as positive
    fn lt_internal(&self, other: &Self) -> bool {
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

    // treats both BNs as positive
    fn le_internal(&self, other: &Self) -> bool {
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

    // treats both BNs as positive
    pub(crate) fn gt_internal(&self, other: &Self) -> bool {
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

    // treats both BNs as positive
    fn ge_internal(&self, other: &Self) -> bool {
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

    // treats both BNs as positive
    fn partial_cmp_internal(&self, other: &Self) -> Option<Ordering> {
        if self.digits.len() != other.digits.len() {
            return Some(self.digits.len().cmp(&other.digits.len()));
        }

        for (s, o) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if s != o {
                return Some(s.cmp(o));
            }
        }

        Some(Ordering::Equal)
    }
}

impl PartialOrd for Bignum {
    fn lt(&self, other: &Self) -> bool {
        match (self.sign, other.sign) {
            // ( x) , ( y)
            (false, false) => Self::lt_internal(self, other),
            // ( x) , (-y)
            (false, true) => false,
            // (-x) , ( y)
            (true, false) => true,
            // (-x) , (-y)
            (true, true) => !Self::lt_internal(self, other),
        }
    }

    fn le(&self, other: &Self) -> bool {
        match (self.sign, other.sign) {
            // ( x) , ( y)
            (false, false) => Self::le_internal(self, other),
            // ( x) , (-y)
            (false, true) => false,
            // (-x) , ( y)
            (true, false) => true,
            // (-x) , (-y)
            (true, true) => !Self::le_internal(self, other),
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match (self.sign, other.sign) {
            // ( x) , ( y)
            (false, false) => Self::gt_internal(self, other),
            // ( x) , (-y)
            (false, true) => true,
            // (-x) , ( y)
            (true, false) => false,
            // (-x) , (-y)
            (true, true) => !Self::gt_internal(self, other),
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match (self.sign, other.sign) {
            // ( x) , ( y)
            (false, false) => Self::ge_internal(self, other),
            // ( x) , (-y)
            (false, true) => true,
            // (-x) , ( y)
            (true, false) => false,
            // (-x) , (-y)
            (true, true) => !Self::ge_internal(self, other),
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.sign, other.sign) {
            // ( x) , ( y)
            (false, false) => Self::partial_cmp_internal(self, other),
            // ( x) , (-y)
            (false, true) => Some(Ordering::Greater),
            // (-x) , ( y)
            (true, false) => Some(Ordering::Less),
            // (-x) , (-y)
            (true, true) => match Self::partial_cmp_internal(self, other) {
                Some(Ordering::Less) => Some(Ordering::Greater),
                Some(Ordering::Greater) => Some(Ordering::Less),
                eq_or_none => eq_or_none,
            },
        }
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
