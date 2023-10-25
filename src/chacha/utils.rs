/// RFC 7539 - Section 2.1
///    1.  `a += b; d ^= a; d <<<= 16;`
///    2.  `c += d; b ^= c; b <<<= 12;`
///    3.  `a += b; d ^= a; d <<<= 8;`
///    4.  `c += d; b ^= c; b <<<= 7;`
pub fn quarter_round_on_vector(a: u32, b: u32, c: u32, d: u32) -> (u32, u32, u32, u32) {
    let (mut a, mut b, mut c, mut d) = (a, b, c, d);
    // 1
    a = a.wrapping_add(b);
    d ^= a;
    d = d.rotate_left(16);
    // 2
    c = c.wrapping_add(d);
    b ^= c;
    b = b.rotate_left(12);
    // 3
    a = a.wrapping_add(b);
    d ^= a;
    d = d.rotate_left(8);
    // 4
    c = c.wrapping_add(d);
    b ^= c;
    b = b.rotate_left(7);

    (a, b, c, d)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// RFC 7539 - Section 2.1.1 - Test Vector for the ChaCha Quarter Round
    fn test_quarter_round_on_vector() {
        let a: u32 = 0x11111111;
        let b: u32 = 0x01020304;
        let c: u32 = 0x9b8d6f43;
        let d: u32 = 0x01234567;

        let (a, b, c, d) = quarter_round_on_vector(a, b, c, d);

        let a_correct: u32 = 0xea2a92f4;
        let b_correct: u32 = 0xcb1cf8ce;
        let c_correct: u32 = 0x4581472e;
        let d_correct: u32 = 0x5881c4bb;

        assert_eq!(a, a_correct);
        assert_eq!(b, b_correct);
        assert_eq!(c, c_correct);
        assert_eq!(d, d_correct);
    }
}
