use crate::rand::Random;
use std::fmt::Write;

pub fn encrypt_password(password: String) -> (String, String) {
    let salt = Random::new(8).generate();
    let password = password + salt.as_str();
    let encrypt_password = md5::compute(password).to_vec();

    let mut result = String::new();
    for v in encrypt_password.iter() {
        write!(result, "{:02x}", v).unwrap()
    }

    (salt, result)
}

pub fn verify_password(password: String, salt: String) -> String {
    let password = password + salt.as_str();
    let encrypt_password = md5::compute(password).to_vec();

    let mut result = String::new();
    for v in encrypt_password.iter() {
        write!(result, "{:02x}", v).unwrap()
    }

    result
}
