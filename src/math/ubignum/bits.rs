use super::bignum::UBignum;

macro_rules! check_pos {
    ($num_digits: expr, $pos: expr) => {
        if $pos >= $num_digits * 64 {
            panic!(
                "Bit index out of bounds. Max index is {} (64 * {} - 1)",
                $num_digits * 64 - 1,
                $num_digits
            );
        }
    };
}

impl<const NUM_DIGITS: usize> UBignum<NUM_DIGITS> {
    #[inline]
    pub fn get_bit(&self, pos: usize) -> bool {
        check_pos!(NUM_DIGITS, pos);

        let chunk_pos = pos / 64;
        let chunk = self.digits[chunk_pos];
        (chunk >> (pos % 64)) & 1 == 1
    }

    #[inline]
    pub fn set_bit(&mut self, pos: usize) {
        check_pos!(NUM_DIGITS, pos);

        let chunk_pos = pos / 64;
        self.digits[chunk_pos] |= 1 << (pos % 64);

        if chunk_pos > self.pos {
            self.pos = chunk_pos;
        }
    }

    #[inline]
    pub fn unset_bit(&mut self, pos: usize) {
        check_pos!(NUM_DIGITS, pos);

        let chunk_pos = pos / 64;
        self.digits[chunk_pos] &= !(1 << (pos % 64));

        if chunk_pos < self.pos {
            return;
        }

        for (i, e) in self.digits[0..self.len()].iter().enumerate().rev() {
            if *e != 0 || i == 0 {
                self.pos = i;
                return;
            }
        }
    }

    #[inline]
    pub fn toggle_bit(&mut self, pos: usize) {
        check_pos!(NUM_DIGITS, pos);

        let chunk_pos = pos / 64;
        self.digits[chunk_pos] ^= 1 << (pos % 64);

        if self.digits[chunk_pos] != 0 {
            // got bigger
            if chunk_pos > self.pos {
                self.pos = chunk_pos;
            }
        } else {
            // got smaller
            for (i, e) in self.digits[0..self.len()].iter().enumerate().rev() {
                if *e != 0 || i == 0 {
                    self.pos = i;
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::ubignum::utils::py_test;

    use super::*;

    #[test]
    fn get_bit() {
        let s = "0x0102030405060708090a0b0c0d0e0fabcdef";
        let bn: UBignum<3> = UBignum::try_from_hex_string(s).unwrap();
        for i in 0..64 * 3 {
            let res1 = bn.get_bit(i) as u64;
            let res2: UBignum<3> = py_test(&format!("({s} >> {i}) & 1"));
            let res2 = res2.digits[0];

            assert_eq!(res1, res2);
        }
    }

    #[test]
    fn set_bit() {
        let s = "0x0102030405060708090a0b0c0d0e0fabcdef0";
        let bn: UBignum<3> = UBignum::try_from_hex_string(s).unwrap();
        for i in 0..64 * 3 {
            let mut bn1: UBignum<3> = bn.clone();
            bn1.set_bit(i);
            let bn2: UBignum<3> = py_test(&format!("{s} | (1 << {i})"));

            assert_eq!(bn1, bn2);
        }
    }

    #[test]
    fn unset_bit1() {
        let s = "0x0102030405060708090a0b0c0d0e0fabcdef0";
        let bn: UBignum<3> = UBignum::try_from_hex_string(s).unwrap();
        for i in 0..64 * 3 {
            let mut bn1: UBignum<3> = bn.clone();
            bn1.unset_bit(i);
            let bn2: UBignum<3> = py_test(&format!("{s} & ~(1 << {i})"));

            assert_eq!(bn1, bn2);
        }
    }

    #[test]
    fn unset_bit2() {
        let s = "0x0100000000000100000000000000010000000";
        let bn: UBignum<3> = UBignum::try_from_hex_string(s).unwrap();
        for i in 0..64 * 3 {
            let mut bn1: UBignum<3> = bn.clone();
            bn1.unset_bit(i);
            let bn2: UBignum<3> = py_test(&format!("{s} & ~(1 << {i})"));

            assert_eq!(bn1, bn2);
        }
    }

    #[test]
    fn toggle_bit1() {
        let s = "0x0102030405060708090a0b0c0d0e0fabcdef0";
        let bn: UBignum<3> = UBignum::try_from_hex_string(s).unwrap();
        for i in 0..64 * 3 {
            let mut bn1: UBignum<3> = bn.clone();
            bn1.toggle_bit(i);
            let bn2: UBignum<3> = py_test(&format!("{s} ^ (1 << {i})"));

            assert_eq!(bn1, bn2);
        }
    }

    #[test]
    fn toggle_bit2() {
        let s = "0x0100000000000100000000000000010000000";
        let bn: UBignum<3> = UBignum::try_from_hex_string(s).unwrap();
        for i in 0..64 * 3 {
            let mut bn1: UBignum<3> = bn.clone();
            bn1.toggle_bit(i);
            let bn2: UBignum<3> = py_test(&format!("{s} ^ (1 << {i})"));

            assert_eq!(bn1, bn2);
        }
    }
}
