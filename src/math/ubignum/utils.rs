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
    UBignum::try_from_hex_string(&s).unwrap()
}
