use nikrypt::math::unsigned_bignum::UnsignedBignum;

#[test]
fn bignum_integration_1() {
    let a = 35;
    let b = 23;
    let c = 100;
    let d = 73;
    let e = 7;

    let big_a = UnsignedBignum::from(a);
    let big_b = UnsignedBignum::from(b);
    let big_c = UnsignedBignum::from(c);
    let big_d = UnsignedBignum::from(d);
    let big_e = UnsignedBignum::from(e);

    let res = (a * b - c + d) / e;
    let res = UnsignedBignum::from(res);

    let res_big = (big_a * big_b - big_c + big_d) / big_e;

    assert_eq!(res, res_big);
}
