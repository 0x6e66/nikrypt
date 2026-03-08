use std::cmp::Ordering;

use crate::math::bignum::Bignum;

#[derive(Debug, PartialEq, Clone)]
struct EccNum(Bignum);

impl EccNum {
    fn zero() -> Self {
        Self(Bignum::zero())
    }
    fn one() -> Self {
        Self(Bignum::one())
    }

    fn add_ref(&self, rhs: &EccNum, p: &EccNum) -> Self {
        let tmp_res = Bignum::add_ref(&self.0, &rhs.0);

        match Bignum::partial_cmp(&tmp_res, &p.0)
            .expect("Can not be None, because of the trait implmentation")
        {
            Ordering::Less => Self(tmp_res),
            Ordering::Equal => Self(Bignum::zero()),
            Ordering::Greater => Self(Bignum::sub_ref(&tmp_res, &p.0)),
        }
    }

    fn sub_ref(&self, rhs: &EccNum, p: &EccNum) -> Self {
        let tmp_res = Bignum::sub_ref(&self.0, &rhs.0);

        match Bignum::partial_cmp(&tmp_res, &Bignum::zero())
            .expect("Can not be None, because of the trait implmentation")
        {
            Ordering::Less => Self(Bignum::add_ref(&tmp_res, &p.0)),
            Ordering::Equal | Ordering::Greater => Self(tmp_res),
        }
    }

    fn mul_ref(&self, rhs: &EccNum, p: &EccNum) -> Self {
        // TODO: find more efficient implementation
        let tmp = Bignum::mul_ref(&self.0, &rhs.0);
        let (_, r) = Bignum::div_with_remainder(&tmp, &p.0);

        Self(r)
    }

    fn get_bit(&self, pos: usize) -> bool {
        self.0.get_bit(pos)
    }
}

impl From<i128> for EccNum {
    fn from(value: i128) -> Self {
        Self(Bignum::from(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::ecc::EccNum;

    #[test]
    fn add_ref() {
        let cases = vec![(13, 57, 101, 70), (13, 9, 23, 22)];

        for case in cases {
            let (a, b, p, res) = case;

            let a = EccNum::from(a);
            let b = EccNum::from(b);
            let p = EccNum::from(p);
            let res = EccNum::from(res);

            let r = EccNum::add_ref(&a, &b, &p);

            assert_eq!(res, r);
        }
    }

    #[test]
    fn sub_ref() {
        let cases = vec![(13, 57, 101, 57), (13, 9, 23, 4)];

        for case in cases {
            let (a, b, p, res) = case;

            let a = EccNum::from(a);
            let b = EccNum::from(b);
            let p = EccNum::from(p);
            let res = EccNum::from(res);

            let r = EccNum::sub_ref(&a, &b, &p);

            assert_eq!(res, r);
        }
    }

    #[test]
    fn mul_ref() {
        let cases = vec![
            (13, 9, 23, 2),
            (13, 57, 101, 34),
            (123, 456, 2003, 4),
            (123, 456, 101, 33),
        ];

        for case in cases {
            let (a, b, p, res) = case;

            let a = EccNum::from(a);
            let b = EccNum::from(b);
            let p = EccNum::from(p);
            let res = EccNum::from(res);

            let r = EccNum::mul_ref(&a, &b, &p);

            assert_eq!(res, r);
        }
    }
}
