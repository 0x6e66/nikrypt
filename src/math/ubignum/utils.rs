use super::bignum::UBignum;

#[inline]
#[allow(dead_code)]
pub(crate) fn py_test<const N: usize>(s: &str) -> UBignum<N> {
    let output = std::process::Command::new("python")
        .arg("-c")
        .arg(format!("print(hex({s}))"))
        .output()
        .expect("Failed to execute command");
    let output = output.stdout.as_slice();
    let s = String::from_utf8(output[0..output.len() - 1].to_vec()).unwrap();
    dbg!(&s);
    UBignum::try_from_hex_string(&s).unwrap()
}

pub(crate) fn get_arithmatik_test_cases() -> Vec<(u128, u128)> {
    let mut test_cases: Vec<(u128, u128)> = vec![(0, 0xa), (0xa, 0), (0, 0)];
    for a in (0..0xabcedef).step_by(5_000_000) {
        for b in (0..0xabcedef).step_by(5_000_000) {
            test_cases.push((a, b));
        }
    }

    test_cases
}

pub(crate) fn check_pos<const NUM_BYTES: usize>(bn: &UBignum<NUM_BYTES>) {
    let mut pos_last_non_zero = 0;
    for (i, e) in bn.digits.iter().enumerate() {
        if *e != 0 {
            pos_last_non_zero = i;
        }
    }

    assert_eq!(pos_last_non_zero, bn.pos);
}
