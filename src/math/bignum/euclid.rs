use crate::math::bignum::Bignum;

impl Bignum {
    pub fn gcd(self, mut b: Self) -> Self {
        let mut a = self;
        while a != b {
            if a > b {
                a = a.sub_ref(&b);
            } else {
                b = b.sub_ref(&a);
            }
        }

        a
    }

    pub fn extended_gcd(self, other: Self) -> (Self, Self, Self) {
        let mut i = self;
        let mut j = other;

        let (mut s, mut t, mut u, mut v) = (Self::one(), Self::zero(), Self::zero(), Self::one());

        while !j.is_zero() {
            let (q, r) = i.div_with_remainder(&j);
            let (unew, vnew) = (s.clone(), t.clone());
            s = u - (q.mul_ref(&s));
            t = v - (q * t);
            (i, j) = (j, r);
            (u, v) = (unew, vnew);
        }
        // d, m, n
        (i, u, v)
    }
}

#[cfg(test)]
mod test {
    use std::ops::Neg;

    use crate::math::bignum::Bignum;

    #[test]
    fn extended_gcd() {
        let a = 34354;
        let b = 1243;

        let d_real = Bignum::from(1);
        let m_real = Bignum::from(10917);
        let n_real = Bignum::from(395).neg();

        let (d, m, n) = Bignum::extended_gcd(a.into(), b.into());

        assert_eq!(d_real, d);
        assert_eq!(m_real, m);
        assert_eq!(n_real, n);
    }
}
