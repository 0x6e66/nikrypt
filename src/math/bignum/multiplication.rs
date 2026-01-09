use crate::math::bignum::Bignum;

impl Bignum {
    // Long Multiplication (https://en.wikipedia.org/wiki/Multiplication_algorithm#Long_multiplication)
    pub fn mul_ref(&self, other: &Self) -> Self {
        let p = self.digits.len();
        let q = other.digits.len();
        let base = u64::MAX as u128 + 1;
        let sign = self.sign ^ other.sign;

        let mut product = vec![0; p + q];

        for b_i in 0..q {
            let mut carry = 0;
            for a_i in 0..p {
                let mut tmp = product[a_i + b_i] as u128;
                tmp += carry + self.digits[a_i] as u128 * other.digits[b_i] as u128;
                carry = tmp / base;
                tmp %= base;
                product[a_i + b_i] = tmp as u64;
            }
            product[b_i + p] = carry as u64;
        }

        let mut tmp = Self {
            digits: product,
            sign,
        };
        tmp.strip();

        tmp
    }
}

impl std::ops::Mul for Bignum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::mul_ref(&self, &rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::math::bignum::{get_test_cases, Bignum};

    #[test]
    fn multiplication() {
        for (a, b) in get_test_cases() {
            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            let res = Bignum::from(a * b);
            let res_big = big_a * big_b;

            assert_eq!(res, res_big);
        }
    }
}
