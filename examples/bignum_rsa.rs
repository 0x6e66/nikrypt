use nikrypt::math::bignum::Bignum;

fn main() {
    let priv_exp = Bignum::try_from_hex_string("0x8367e1ba7e06c57060c8fbebccb8b033a3c8105b30d7dc31b2e7d1e97dae1ec75b4f5fb0f9f3c9c160fe257d68d74495eea80c0af838f37c9db7a24558c21e28c49d57470b002d90a383caebb5821a59583d15502f0012c9235f806c62c97f1e3cafbc72118fcf60743168125801e06cc7293cde64d241339aad516e7bcc1081").unwrap();
    let pub_exp = Bignum::try_from_hex_string("0x10001").unwrap();

    let p = Bignum::try_from_hex_string("0xd00e8de65f7c32094b732a5628cefedc35ed796b7cea6297614545df71d8dbc67ea14565534bfc9bc5f1a680239227189c2d493924a5bd64641169533201d6e5").unwrap();
    let q = Bignum::try_from_hex_string("0xcd3ac881cc47aa776a9829c0e529e3d0dbb5a43c366842578341a051dafaf4f6164f2dc0e72a3bd3b33f8b2f84a6cf35f0781e7c466e677ff8e553de5c92c617").unwrap();
    let n = Bignum::mul_ref(&p, &q);

    let msg = Bignum::try_from_hex_string("0x414141414141414141").unwrap();
    println!("Mesage to enc: '{}'", msg.to_hex_string());

    let cip = enc(msg.clone(), pub_exp, n.clone());
    println!("Encrypted message: '{}'", cip.to_hex_string());

    let clear = dec(cip, priv_exp, n);
    println!("Encrypted message: '{}'", clear.to_hex_string());

    assert_eq!(clear, msg);
}

fn enc(msg: Bignum, exp: Bignum, modu: Bignum) -> Bignum {
    Bignum::pow_mod(msg, exp, &modu)
}

fn dec(ciph: Bignum, exp: Bignum, modu: Bignum) -> Bignum {
    Bignum::pow_mod(ciph, exp, &modu)
}
