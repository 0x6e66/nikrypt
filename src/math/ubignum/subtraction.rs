use super::bignum::UBignum;

impl<const NUM_DIGITS: usize> UBignum<NUM_DIGITS> {
    pub fn sub_assign_ref(&mut self, rhs: &Self) {
        if *self < *rhs {
            panic!(
                "Result of subtraction would be negative.\nlhs: {}\nrhs: {}",
                self.to_hex_string(),
                rhs.to_hex_string()
            );
        } else if self == rhs {
            self.set_zero();
            return;
        }

        let mut carry: u8 = 0;

        for (left, right) in self.digits.iter_mut().rev().zip(rhs.digits.iter().rev()) {
            let (res, c1) = left.overflowing_sub(*right);
            let (res, c2) = res.overflowing_sub(carry as u64);
            *left = res;
            carry = (c1 || c2).into();
        }
    }
}

impl<const NUM_DIGITS: usize> std::ops::SubAssign for UBignum<NUM_DIGITS> {
    fn sub_assign(&mut self, rhs: Self) {
        Self::sub_assign_ref(self, &rhs);
    }
}

#[cfg(test)]
mod tests {
    use crate::math::ubignum::utils::{get_arithmatik_test_cases, py_test};

    use super::*;

    #[test]
    fn subtraction1() {
        for (a, b) in get_arithmatik_test_cases() {
            if a < b {
                continue;
            }
            let mut bn_a: UBignum<2> = UBignum::from(a);
            let bn_b: UBignum<2> = UBignum::from(b);

            let bn_res: UBignum<2> = py_test(&format!(
                "{}-{}",
                bn_a.to_hex_string(),
                bn_b.to_hex_string()
            ));

            bn_a.sub_assign_ref(&bn_b);
        }
    }

    #[test]
    #[should_panic]
    fn subtraction2() {
        for (a, b) in get_arithmatik_test_cases() {
            if a >= b {
                continue;
            }
            let mut bn_a: UBignum<2> = UBignum::from(a);
            let bn_b: UBignum<2> = UBignum::from(b);

            bn_a.sub_assign_ref(&bn_b);
        }
    }
}
