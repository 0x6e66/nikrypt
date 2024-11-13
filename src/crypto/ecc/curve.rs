use crate::math::{unsigned_bignum_fast::UnsignedBignumFast, utils::egcd};

#[derive(Debug, Clone)]
pub struct Curve<const NUM_BYTES: usize> {
    id: String,
    p: UnsignedBignumFast<NUM_BYTES>,
    a: UnsignedBignumFast<NUM_BYTES>,
    b: UnsignedBignumFast<NUM_BYTES>,
    g: EccPoint<NUM_BYTES>,
    q: UnsignedBignumFast<NUM_BYTES>,
    h: usize,
}

impl<const NUM_BYTES: usize> Curve<NUM_BYTES> {
    pub fn get_random_point(&self) -> EccPoint<NUM_BYTES> {
        let x: UnsignedBignumFast<NUM_BYTES> = UnsignedBignumFast::rand();
        let (_, x) = UnsignedBignumFast::div_with_remainder(&x, &self.p);
        let x = EccCoordinate { bn: x };

        let y: UnsignedBignumFast<NUM_BYTES> = UnsignedBignumFast::rand();
        let (_, y) = UnsignedBignumFast::div_with_remainder(&y, &self.p);
        let y = EccCoordinate { bn: y };

        EccPoint { x, y }
    }
}

// #############################################################

#[derive(Debug, Clone)]
pub struct EccPoint<const NUM_BYTES: usize> {
    x: EccCoordinate<NUM_BYTES>,
    y: EccCoordinate<NUM_BYTES>,
}

// #############################################################

#[derive(Debug, Clone)]
pub struct EccCoordinate<const NUM_BYTES: usize> {
    bn: UnsignedBignumFast<NUM_BYTES>,
}

impl<const NUM_BYTES: usize> EccCoordinate<NUM_BYTES> {
    pub fn new() -> Self {
        Self::zero()
    }

    pub fn zero() -> Self {
        Self {
            bn: UnsignedBignumFast::zero(),
        }
    }

    pub fn add_ref(&self, rhs: &Self, p: &UnsignedBignumFast<NUM_BYTES>) -> Self {
        let tmp = self.bn.add_ref(&rhs.bn);
        if tmp < *p {
            Self { bn: tmp }
        } else if tmp == *p {
            Self {
                bn: UnsignedBignumFast::new(),
            }
        } else {
            Self { bn: tmp.sub_ref(p) }
        }
    }

    pub fn sub_ref(&self, rhs: &Self, p: &UnsignedBignumFast<NUM_BYTES>) -> Self {
        if self.bn > rhs.bn {
            Self {
                bn: self.bn.sub_ref(&rhs.bn),
            }
        } else if self.bn < rhs.bn {
            Self {
                bn: self.bn.add_ref(&p).sub_ref(&rhs.bn),
            }
        } else {
            Self::new()
        }
    }

    pub fn mul_ref(&self, rhs: &Self, p: &UnsignedBignumFast<NUM_BYTES>) -> Self {
        let (_, r) = self.bn.mul_ref(&rhs.bn).div_with_remainder(p);
        Self { bn: r }
    }

    pub fn div_ref(&self, rhs: &Self, p: &UnsignedBignumFast<NUM_BYTES>) -> Self {
        let (_, mut m, _) = egcd(rhs.bn.clone().into(), p.clone().into());

        if m.sign {
            m = m.add_ref(&p.clone().into());
            m.sign = false;
        }

        let m_coord = Self { bn: m.into() };
        let res = self.mul_ref(&m_coord, p);

        res
    }

    pub fn pow(&self, rhs: &Self, p: &UnsignedBignumFast<NUM_BYTES>) -> Self {
        let res = self.bn.clone().pow_mod(rhs.bn.clone(), p);
        Self { bn: res }
    }

    pub fn from_u128(value: u128, p: &UnsignedBignumFast<NUM_BYTES>) -> Self {
        let tmp: UnsignedBignumFast<NUM_BYTES> = UnsignedBignumFast::from(value);
        let (_, r) = UnsignedBignumFast::div_with_remainder(&tmp, p);

        Self { bn: r }
    }
}

impl<const NUM_BYTES: usize> PartialEq for EccCoordinate<NUM_BYTES> {
    fn eq(&self, other: &Self) -> bool {
        self.bn.eq(&other.bn)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const N: usize = 20;

    fn get_test_cases() -> Vec<(u128, u128, u128)> {
        let mut test_cases = vec![];
        for a in (0..0xabcd).step_by(5_000) {
            for b in (0..0xabcd).step_by(5_000) {
                for p in (1..0xabcd).step_by(5_000) {
                    test_cases.push((a, b, p));
                }
            }
        }

        test_cases
    }

    #[test]
    fn ecc_coordinate_addition() {
        for (a, b, p) in get_test_cases() {
            let big_p = UnsignedBignumFast::from(p);
            let coord_a: EccCoordinate<N> = EccCoordinate::from_u128(a, &big_p);
            let coord_b: EccCoordinate<N> = EccCoordinate::from_u128(b, &big_p);

            let res = EccCoordinate::from_u128((a + b) % p, &big_p);
            let big_res = EccCoordinate::add_ref(&coord_a, &coord_b, &big_p);

            assert_eq!(res, big_res);
        }
    }

    #[test]
    fn ecc_coordinate_subtraction() {
        for (a, b, p) in get_test_cases() {
            let big_p = UnsignedBignumFast::from(p);
            let coord_a: EccCoordinate<N> = EccCoordinate::from_u128(a, &big_p);
            let coord_b: EccCoordinate<N> = EccCoordinate::from_u128(b, &big_p);

            let a = a % p;
            let b = b % p;

            let tmp_res = match a >= b {
                true => a - b,
                false => (a + p) - b,
            };
            let res = EccCoordinate::from_u128(tmp_res, &big_p);
            let big_res = EccCoordinate::sub_ref(&coord_a, &coord_b, &big_p);

            assert_eq!(res, big_res);
        }
    }

    #[test]
    fn ecc_coordinate_multiplication() {
        for (a, b, p) in get_test_cases() {
            let big_p = UnsignedBignumFast::from(p);
            let coord_a: EccCoordinate<N> = EccCoordinate::from_u128(a, &big_p);
            let coord_b: EccCoordinate<N> = EccCoordinate::from_u128(b, &big_p);

            let a = a % p;
            let b = b % p;

            let res = EccCoordinate::from_u128((a * b) % p, &big_p);
            let big_res = EccCoordinate::mul_ref(&coord_a, &coord_b, &big_p);

            assert_eq!(res, big_res);
        }
    }

    #[test]
    fn ecc_coordinate_division() {
        for (a, b, r, p) in [
            (77, 13, 37, 101),
            (13, 77, 71, 101),
            (123, 1234, 1758, 2003),
            (123412409, 12901825, 1164, 4337),
        ] {
            if b == 0 {
                continue;
            }
            let big_p = UnsignedBignumFast::from(p);
            let coord_a: EccCoordinate<N> = EccCoordinate::from_u128(a, &big_p);
            let coord_b: EccCoordinate<N> = EccCoordinate::from_u128(b, &big_p);

            let res = EccCoordinate::from_u128(r, &big_p);
            let big_res = EccCoordinate::div_ref(&coord_a, &coord_b, &big_p);

            assert_eq!(res, big_res);
        }
    }
}
