#[cfg(test)]
use std::str::FromStr;

use num_bigint::BigUint;

pub fn encrypt_pass(password: String) -> String {
    let e = 65537u32;
    let n = BigUint::parse_bytes(b"94dd2a8675fb779e6b9f7103698634cd400f27a154afa67af6166a43fc26417222a79506d34cacc7641946abda1785b7acf9910ad6a0978c91ec84d40b71d2891379af19ffb333e7517e390bd26ac312fe940c340466b4a5d4af1d65c3b5944078f96a1a51a5a53e4bc302818b7c9f63c4a1b07bd7d874cef1c3d4b2f5eb7871",16).unwrap();
    let c = BigUint::from_bytes_be(&password.as_bytes());
    let result = c.modpow(&BigUint::from(e), &n);
    format!("{:0>256}", result.to_str_radix(16))
}

#[test]
fn encrypt_test() {
    assert_eq!(encrypt_pass(String::from_str("123>5ae915bf808f82732e98e01f704f00cd").unwrap()),"91a0e02175f6a0b22ad23dac0d7f599806bc091f9fee1bfdada0d24d011dcdaed418296b7c0ec560f988d92a7bb25dbf7ff51752d9bc6482a8180e56f7b772079ab59844abaae91e6d1c4660dc872717f9218f89acc9b70bb32891f28bf9d8f173d81b0e36c828deac919783e4e909ad1c22f953947b4a7ed7c90ac18fd95aa2");
    assert_eq!(encrypt_pass(String::from_str("197").unwrap()),"0038c3a7a9719b65a89b82f56bdfc62c71f646403e169fbe1a391d8d1468e648e65e833174db7f1fad21e609ebd21432739e8ee7a3758938b4bd1d07390064918cf1763d6853525b761b055ae3dc229b1579eeacb7281ab258f2ea5c27455861503d814adb857000b24267fca4e70cac4e618f6258367367c0e43c2518e032d8");
}
