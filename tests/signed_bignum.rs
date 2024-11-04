use nikrypt::math::signed_bignum::SignedBignum;

#[test]
fn bignum_integration_1() {
    let a = -35;
    let b = -23;
    let c = -100;
    let d = -73;
    let e = -7;

    let big_a = SignedBignum::from(a);
    let big_b = SignedBignum::from(b);
    let big_c = SignedBignum::from(c);
    let big_d = SignedBignum::from(d);
    let big_e = SignedBignum::from(e);

    let res = (a * b - c + d) / e;
    let res = SignedBignum::from(res);

    let res_big = (big_a * big_b - big_c + big_d) / big_e;

    assert_eq!(res, res_big);
}
