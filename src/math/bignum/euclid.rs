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
    use crate::math::bignum::Bignum;

    #[test]
    fn extended_gcd() {
        let a = 934354;
        let b = 12433;

        let (d, m, n) = Bignum::extended_gcd(a.into(), b.into());

        dbg!(a, b, d, m, n);
        assert!(false);
    }
}
