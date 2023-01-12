use std::fmt::Write;
use crate::rand::CharSetKind::NumberAndLetter;
use crate::rand::Custom;

pub fn encrypt_password(password: String) -> (String, String) {
    let salt = Custom::new(8,NumberAndLetter).generate();
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
