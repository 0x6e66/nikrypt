use crate::math::bignum_fast::BignumFast;

#[derive(Debug, Clone)]
pub struct Curve<const NUM_BYTES: usize> {
    id: String,
    p: BignumFast<NUM_BYTES>,
    a: BignumFast<NUM_BYTES>,
    b: BignumFast<NUM_BYTES>,
    g: EccPoint<NUM_BYTES>,
    q: BignumFast<NUM_BYTES>,
    h: usize,
}

impl<const NUM_BYTES: usize> Curve<NUM_BYTES> {
    pub fn get_random_point(&self) -> EccPoint<NUM_BYTES> {
        let x: BignumFast<NUM_BYTES> = BignumFast::rand();
        let (_, x) = BignumFast::div_with_remainder(&x, &self.p);
        let x = EccCoordinate { bn: x };

        let y: BignumFast<NUM_BYTES> = BignumFast::rand();
        let (_, y) = BignumFast::div_with_remainder(&y, &self.p);
        let y = EccCoordinate { bn: y };

        EccPoint { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct EccPoint<const NUM_BYTES: usize> {
    x: EccCoordinate<NUM_BYTES>,
    y: EccCoordinate<NUM_BYTES>,
}

#[derive(Debug, Clone)]
pub struct EccCoordinate<const NUM_BYTES: usize> {
    bn: BignumFast<NUM_BYTES>,
}

impl<const NUM_BYTES: usize> EccCoordinate<NUM_BYTES> {
    pub fn new() -> Self {
        Self::zero()
    }

    pub fn zero() -> Self {
        Self {
            bn: BignumFast::zero(),
        }
    }

    pub fn add_ref(&self, rhs: &Self, p: &BignumFast<NUM_BYTES>) -> Self {
        let tmp = self.bn.add_ref(&rhs.bn);
        if tmp < *p {
            Self { bn: tmp }
        } else if tmp == *p {
            Self {
                bn: BignumFast::new(),
            }
        } else {
            Self { bn: tmp.sub_ref(p) }
        }
    }

    pub fn sub_ref(&self, rhs: &Self, p: &BignumFast<NUM_BYTES>) -> Self {
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

    pub fn from_u128(value: u128, p: &BignumFast<NUM_BYTES>) -> Self {
        let tmp: BignumFast<NUM_BYTES> = BignumFast::from(value);
        let (_, r) = BignumFast::div_with_remainder(&tmp, p);

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

    const N: usize = 200;

    #[test]
    fn ecc_coordinate_addition() {
        let mut test_cases = vec![];
        for a in (0..0xabcd).step_by(5_000) {
            for b in (0..0xabcd).step_by(5_000) {
                for p in (1..0xabcd).step_by(5_000) {
                    test_cases.push((a, b, p));
                }
            }
        }

        for (a, b, p) in test_cases {
            let big_p = BignumFast::from(p);
            let coord_a: EccCoordinate<N> = EccCoordinate::from_u128(a, &big_p);
            let coord_b: EccCoordinate<N> = EccCoordinate::from_u128(b, &big_p);

            let res = EccCoordinate::from_u128((a + b) % p, &big_p);
            let big_res = EccCoordinate::add_ref(&coord_a, &coord_b, &big_p);

            assert_eq!(res, big_res);
        }
    }
}
