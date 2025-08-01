use crate::math::bignum::Bignum;

impl Bignum {
    pub fn sub_ref(&self, rhs: &Self) -> Self {
        if self < rhs {
            panic!(
                "Result of subtraction would be negative.\nlhs: {}\nrhs: {}",
                self.to_hex_string(),
                rhs.to_hex_string()
            );
        }

        let (long, short) = match self > rhs {
            true => (self, rhs),
            false => (rhs, self),
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

        let mut res = Self { digits: vec };
        res.strip();

        res
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
        for (a, b) in get_test_cases() {
            let (a, b) = match a >= b {
                true => (a, b),
                false => (b, a),
            };

            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            let res = Bignum::from(a - b);
            let res_big = big_a - big_b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    #[should_panic]
    fn subtraction_panic() {
        for (a, b) in get_test_cases() {
            let (a, b) = match a > b {
                false => (a, b),
                true => (b, a),
            };

            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            // should panic here
            let _res_big = big_a - big_b;
        }
    }
}
