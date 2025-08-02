use crate::math::bignum::Bignum;

impl Bignum {
    pub fn get_bit(&self, pos: usize) -> bool {
        if pos >= self.digits.len() * 64 {
            return false;
        }

        let byte = self.digits[pos / 64];
        (byte >> (pos % 64)) & 1 == 1
    }

    pub fn set_bit(&mut self, pos: usize) {
        if pos >= self.digits.len() * 64 {
            self.digits.resize(pos / 64 + 1, 0);
        }

        self.digits[pos / 64] |= 1 << (pos % 64);
    }

    pub fn unset_bit(&mut self, pos: usize) {
        if pos >= self.digits.len() * 64 {
            return;
        }

        self.digits[pos / 64] &= !(1 << (pos % 64));
    }

    pub fn toggle_bit(&mut self, pos: usize) {
        if pos >= self.digits.len() * 64 {
            self.digits.resize(pos / 64 + 1, 0);
        }

        self.digits[pos / 64] ^= 1 << (pos % 64);
    }
}

impl std::ops::Shr<usize> for Bignum {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        let new_len = self.len() - self.digits.len().saturating_sub(rhs / 64);
        let bit_shift = (rhs % 64) as u64;

        for _ in 0..new_len {
            self.digits.remove(0);
        }

        if bit_shift == 0 {
            self.strip();
            return self;
        }

        let mut carry = 0;
        for b in self.digits.iter_mut().rev() {
            let tmp_carry = *b << (64 - bit_shift);
            *b >>= bit_shift;
            *b |= carry;
            carry = tmp_carry;
        }

        self.strip();

        self
    }
}

impl std::ops::Shl<usize> for Bignum {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        let byte_shift = rhs / 64;
        let shift = (rhs % 64) as u64;

        for _ in 0..byte_shift {
            self.digits.insert(0, 0);
        }

        if shift == 0 {
            return self;
        }

        self.digits.push(0);

        let mut carry = 0;
        for b in self.digits.iter_mut() {
            let tmp_carry = *b >> (64 - shift);
            *b <<= shift;
            *b |= carry;
            carry = tmp_carry;
        }

        self.strip();

        self
    }
}

#[cfg(test)]
mod test {
    use crate::math::bignum::Bignum;

    const BASE: u128 = 0xabcedefabcedefabcedefabcedefabcd;

    #[test]
    fn shift_right() {
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..128 {
            test_cases.push((BASE, i));
        }

        for (a, b) in test_cases {
            let big_a = Bignum::from(a);

            let res = Bignum::from(a >> b);
            let res_big = big_a >> b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn shift_left() {
        let base = 0xabcedefabcedefabcede;
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..(12 * 4) {
            test_cases.push((base, i));
        }

        for (a, b) in test_cases {
            let big_a = Bignum::from(a);

            let res = Bignum::from(a << b);
            let res_big = big_a << b;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn get_bit() {
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..128 {
            test_cases.push((BASE, i));
        }

        for (a, b) in test_cases {
            let big_a = Bignum::from(a);
            let res_big = big_a.get_bit(b);

            let res = (a >> b) & 1 == 1;

            assert_eq!(res, res_big);
        }
    }

    #[test]
    fn set_bit() {
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..128 {
            test_cases.push((BASE, i));
        }

        for (mut a, b) in test_cases {
            let mut big_a = Bignum::from(a);
            big_a.set_bit(b);

            a |= 1 << b;
            let a = Bignum::from(a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn unset_bit() {
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..128 {
            test_cases.push((BASE, i));
        }

        for (mut a, b) in test_cases {
            let mut big_a = Bignum::from(a);
            big_a.unset_bit(b);

            a &= !(1 << b);
            let a = Bignum::from(a);

            assert_eq!(a, big_a);
        }
    }

    #[test]
    fn toggle_bit() {
        let mut test_cases: Vec<(u128, usize)> = vec![];
        for i in 0..128 {
            test_cases.push((BASE, i));
        }

        for (mut a, b) in test_cases {
            let mut big_a = Bignum::from(a);
            big_a.toggle_bit(b);

            a ^= 1 << b;
            let a = Bignum::from(a);

            assert_eq!(a, big_a);
        }
    }
}
