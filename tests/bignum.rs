use nikrypt::math::bignum::Bignum;

#[test]
fn bignum_rsa() {
    let priv_exp = Bignum::try_from_hex_string("0x8367e1ba7e06c57060c8fbebccb8b033a3c8105b30d7dc31b2e7d1e97dae1ec75b4f5fb0f9f3c9c160fe257d68d74495eea80c0af838f37c9db7a24558c21e28c49d57470b002d90a383caebb5821a59583d15502f0012c9235f806c62c97f1e3cafbc72118fcf60743168125801e06cc7293cde64d241339aad516e7bcc1081").unwrap();
    let pub_exp = Bignum::try_from_hex_string("0x10001").unwrap();

    let p = Bignum::try_from_hex_string("0xd00e8de65f7c32094b732a5628cefedc35ed796b7cea6297614545df71d8dbc67ea14565534bfc9bc5f1a680239227189c2d493924a5bd64641169533201d6e5").unwrap();
    let q = Bignum::try_from_hex_string("0xcd3ac881cc47aa776a9829c0e529e3d0dbb5a43c366842578341a051dafaf4f6164f2dc0e72a3bd3b33f8b2f84a6cf35f0781e7c466e677ff8e553de5c92c617").unwrap();
    let n = Bignum::mul_ref(&p, &q);

    let msg = Bignum::try_from_hex_string("0x414141414141414141").unwrap();

    let cip = Bignum::pow_mod(msg.clone(), pub_exp, &n);
    assert_eq!(cip.to_hex_string().as_str(), "0x55f46e9050fa9dc8dd854cb03dd4bef2a57a84e1e55bc7519e2154542d6be4f845e35a1f095aa8f427fd6d9263ace6ed1d7ce1dc8cb76ea61c82b25a55ddf7f6f701a9a3feb173ec0737492e2fb9506f8af90a018bc1f46423f6f4d496412ae5c967020b3e371ef2a3500c087b677d7ae92fdd4614ba2eba3127ec65db6ae1");

    let clear = Bignum::pow_mod(cip, priv_exp, &n);
    assert_eq!(clear, msg);
}

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
