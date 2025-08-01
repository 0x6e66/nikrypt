use crate::math::bignum::Bignum;

impl Bignum {
    /// Integer division (unsigned) with remainder (https://en.wikipedia.org/wiki/Division_algorithm#Integer_division_(unsigned)_with_remainder)
    /// returns (quotient, remainder)
    pub fn div_with_remainder(&self, rhs: &Self) -> (Self, Self) {
        let mut quotient = Self::new();
        let mut remainder = Self::new();

        let (n_len, n) = (self.digits.len() * 64, self);

        for i in (0..n_len).rev() {
            remainder = remainder << 1;
            if n.get_bit(i) {
                remainder.set_bit(0);
            } else {
                remainder.unset_bit(0);
            }

            if remainder >= *rhs {
                remainder = remainder.sub_ref(rhs);
                quotient.set_bit(i);
            }
        }

        (quotient, remainder)
    }
}

impl std::ops::Div for Bignum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let (q, _) = self.div_with_remainder(&rhs);
        q
    }
}

impl std::ops::Rem for Bignum {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let (_, r) = self.div_with_remainder(&rhs);
        r
    }
}

#[cfg(test)]
mod test {
    use crate::math::bignum::{get_test_cases, Bignum};

    #[test]
    fn division_with_remainder() {
        for (a, b) in get_test_cases() {
            if b == 0 {
                continue;
            }

            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            let (big_q, big_r) = Bignum::div_with_remainder(&big_a, &big_b);
            let q = Bignum::from(a / b as u128);
            let r = Bignum::from(a % b as u128);

            assert_eq!(big_q, q);
            assert_eq!(big_r, r);
        }
    }
}
