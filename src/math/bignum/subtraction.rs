use crate::math::bignum::Bignum;

impl Bignum {
    // treats both BNs as positive
    pub(crate) fn sub_ref_internal(&self, rhs: &Self) -> Self {
        if self == rhs {
            return Self::zero();
        }

        let (long, short, sign) = match self.gt_internal(rhs) {
            true => (self, rhs, false),
            false => (rhs, self, true),
        };
        let mut vec = vec![0u64; long.len()];

        let mut carry = 0;
        for (i, e) in vec.iter_mut().enumerate() {
            let (mut sum, mut tmp_carry) = long.digits[i].overflowing_sub(carry);
            carry = tmp_carry as u64;

            if i < short.len() {
                (sum, tmp_carry) = sum.overflowing_sub(short.digits[i]);
                carry += tmp_carry as u64;
            }

            *e = sum;
        }

        let mut res = Self { digits: vec, sign };
        res.strip();

        res
    }

    pub fn sub_ref(&self, rhs: &Self) -> Self {
        match (self.sign, rhs.sign) {
            // ( x) - ( y)
            (false, false) => Self::sub_ref_internal(self, rhs),

            // ( x) - (-y) => x + y
            (false, true) => Self::add_ref_internal(self, rhs),

            // (-x) - ( y) => -x - y => -(x+y)
            (true, false) => -Self::add_ref_internal(self, rhs),

            // (-x) - (-y) => y - x
            (true, true) => -Self::sub_ref_internal(rhs, self),
        }
    }
}

impl std::ops::Sub for Bignum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_ref(&rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::math::bignum::{get_test_cases, Bignum};

    #[test]
    fn subtraction() {
        for (mut a, mut b) in get_test_cases() {
            if a < b {
                (a, b) = (b, a);
            }

            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            let res = Bignum::from(a - b);
            let res_big = big_a - big_b;

            assert_eq!(res, res_big);
        }
    }
}
