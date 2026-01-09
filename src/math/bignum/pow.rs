use crate::math::bignum::Bignum;

impl Bignum {
    /// Exponentiation by squaring (https://en.wikipedia.org/wiki/Exponentiation_by_squaring)
    pub fn pow(self, other: Self) -> Self {
        let mut x = self;
        let mut n = other;

        if n.is_signed() {
            panic!("Attempted potentiation with negative exponent. Floating point operations are not supported.");
        }

        let sign = x.sign;
        x.unset_sign();

        let one = 1.into();
        let two = 2.into();

        if n.is_zero() {
            return one;
        }

        let mut y = one.clone();
        while n > 1.into() {
            if !n.is_even() {
                y = x.mul_ref(&y);
                n = n.sub_ref(&one);
            }
            x = x.mul_ref(&x);
            (n, _) = n.div_with_remainder(&two);
        }

        let mut res = x * y;
        res.sign = sign;

        res
    }

    pub fn pow_mod(self, exponent: Self, modulus: &Self) -> Self {
        let mut base = self;
        let mut exp = exponent;

        if exp.is_signed() {
            panic!("Attempted potentiation with negative exponent. Floating point operations are not supported.");
        }

        let sign = base.sign;

        let mut t = Self::from(1);
        while !exp.is_zero() {
            if !exp.is_even() {
                (_, t) = Self::mul_ref(&t, &base).div_with_remainder(modulus);
            }
            (_, base) = Self::mul_ref(&base, &base).div_with_remainder(modulus);
            exp = exp >> 1;
        }

        let (_, mut r) = t.div_with_remainder(modulus);
        r.sign = sign;

        r
    }
}

#[cfg(test)]
mod test {
    use crate::math::bignum::Bignum;

    #[test]
    fn pow() {
        let mut test_cases: Vec<(u128, u128)> = vec![(0, 0xa), (0xa, 0), (0, 0)];
        for a in 0..20 {
            for b in 0..20 {
                test_cases.push((a, b));
            }
        }

        for (a, b) in test_cases {
            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            let res = Bignum::from(a.pow(b as u32));
            let res_big = big_a.pow(big_b);

            assert_eq!(res, res_big);
        }
    }
}
