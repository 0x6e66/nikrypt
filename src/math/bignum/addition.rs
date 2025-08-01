use crate::math::bignum::Bignum;

impl Bignum {
    pub fn add_ref(&self, rhs: &Self) -> Self {
        let (long, short) = match self.len() > rhs.len() {
            true => (self, rhs),
            false => (rhs, self),
        };
        let mut vec = vec![0u64; long.len()];

        let mut carry = 0;
        for (i, e) in vec.iter_mut().enumerate() {
            let mut tmp = long.digits[i] as u128 + carry;
            if i < short.len() {
                tmp += short.digits[i] as u128;
            }
            carry = tmp >> 64;

            *e = tmp as u64;
        }

        if carry != 0 {
            vec.push(carry as u64);
        }

        Self { digits: vec }
    }
}

impl std::ops::Add for Bignum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_ref(&rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::math::bignum::{get_test_cases, Bignum};

    #[test]
    fn addition() {
        for (a, b) in get_test_cases() {
            let big_a = Bignum::from(a);
            let big_b = Bignum::from(b);

            let res = Bignum::from(a + b);
            let res_big = big_a + big_b;

            dbg!(a, b);
            assert_eq!(res, res_big);
        }
    }
}
