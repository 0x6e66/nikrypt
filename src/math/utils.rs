use crate::math::bignum::Bignum;

pub fn gcd(a: Bignum, b: Bignum) -> Bignum {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        for (a, b, c) in [
            (18, 24, 6),
            (12375, 8975, 25),
            (0xaabbcc, 0xddeeff, 0x99),
            (0xaabb, 0xddee, 0x33),
        ] {
            let a = Bignum::from(a);
            let b = Bignum::from(b);
            let c = Bignum::from(c);

            let res = gcd(a, b);

            assert_eq!(res, c);
        }
    }
}
