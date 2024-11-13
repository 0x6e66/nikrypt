use crate::math::unsigned_bignum::UnsignedBignum;

use super::signed_bignum_fast::SignedBignumFast;

pub fn gcd(a: UnsignedBignum, b: UnsignedBignum) -> UnsignedBignum {
    let mut a = a;
    let mut b = b;

    loop {
        let (_, temp) = a.div_with_remainder(&b);
        if temp.is_zero() {
            return b;
        }

        a = b;
        b = temp;
    }
}

pub fn egcd<const N: usize>(
    mut d: SignedBignumFast<N>,
    mut b: SignedBignumFast<N>,
) -> (
    SignedBignumFast<N>,
    SignedBignumFast<N>,
    SignedBignumFast<N>,
) {
    let (mut m, mut n, mut sb, mut tb) = (
        SignedBignumFast::from(1),
        SignedBignumFast::from(0),
        SignedBignumFast::from(0),
        SignedBignumFast::from(1),
    );
    while !b.is_zero() {
        let (q, r) = d.div_with_remainder(&b);
        (sb, tb, m, n) = (m - q.mul_ref(&sb), n - q.mul_ref(&tb), sb, tb);
        (d, b) = (b, r);
    }
    (d, m, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: usize = 20;

    #[test]
    fn gcd_test() {
        for (a, b, c) in [
            (18, 24, 6),
            (12375, 8975, 25),
            (0xaabbcc, 0xddeeff, 0x99),
            (0xaabb, 0xddee, 0x33),
        ] {
            let a = UnsignedBignum::from(a);
            let b = UnsignedBignum::from(b);
            let c = UnsignedBignum::from(c);

            let res = gcd(a, b);

            assert_eq!(res, c);
        }
    }

    #[test]
    fn egcd_test() {
        for (a, b, d, m, n) in [
            (101, 13, 1, 4, -31),
            (101, 77, 1, -16, 21),
            (2003, 1234, 1, -69, 112),
            (5719087, 2938457, 1, -614308, 1195621),
        ] {
            let a: SignedBignumFast<N> = SignedBignumFast::from(a);
            let b = SignedBignumFast::from(b);
            let d = SignedBignumFast::from(d);
            let m = SignedBignumFast::from(m);
            let n = SignedBignumFast::from(n);

            let egcd = egcd(a, b);
            assert_eq!((d, m, n), egcd);
        }
    }
}
