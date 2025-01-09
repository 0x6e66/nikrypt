use super::bignum::UBignum;

impl<const NUM_DIGITS: usize> UBignum<NUM_DIGITS> {
    pub fn add_assign_ref(&mut self, rhs: &Self) -> u8 {
        let mut carry: u8 = 0;
        for (left, right) in self.digits.iter_mut().rev().zip(rhs.digits.iter().rev()) {
            let (res, c1) = left.overflowing_add(*right);
            let (res, c2) = res.overflowing_add(carry as u64);
            *left = res;
            carry = (c1 || c2).into();
        }
        carry
    }
}

#[cfg(test)]
mod tests {
    use crate::math::ubignum::utils::py_test;

    use super::*;

    #[test]
    fn addition() {
        for a in (0..usize::MAX).step_by(usize::MAX / 10) {
            for b in (0..usize::MAX).step_by(usize::MAX / 10) {
                let mut bn_a: UBignum<2> = UBignum::from(a);
                let bn_b: UBignum<2> = UBignum::from(b);

                bn_a.add_assign_ref(&bn_b);

                let bn_res: UBignum<2> = py_test(&format!(
                    "{}+{}",
                    bn_a.to_hex_string(),
                    bn_b.to_hex_string()
                ));
            }
        }
    }
}
