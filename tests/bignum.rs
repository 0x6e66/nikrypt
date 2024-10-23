use nikrypt::math::bignum::Bignum;

#[test]
fn bignum_integration_1() {
    let a = 35;
    let b = 23;
    let c = 100;
    let d = 73;
    let e = 7;

    let big_a = Bignum::from(a);
    let big_b = Bignum::from(b);
    let big_c = Bignum::from(c);
    let big_d = Bignum::from(d);
    let big_e = Bignum::from(e);

    let res = (a * b - c + d) / e;
    let res = Bignum::from(res);

    let res_big = (big_a * big_b - big_c + big_d) / big_e;

    assert_eq!(res, res_big);
}
