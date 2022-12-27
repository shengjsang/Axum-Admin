use crate::rand::Random;
use crate::redis::{connect, set};
use anyhow::Result;
use captcha::filters::Noise;
use captcha::Captcha;

pub async fn new() -> Result<(String, String)> {
    let (captcha_id, captcha_code, captcha_img) = generate();
    let mut con = connect().await?;
    set(
        &mut con,
        captcha_id.as_str(),
        captcha_code.as_str(),
        60 * 15,
    )
    .await?;

    Ok((captcha_id, captcha_img))
}

pub fn generate() -> (String, String, String) {
    let captcha_id = Random::new(12).generate();

    let mut captcha = Captcha::new();
    let captcha_code = captcha.add_chars(6).chars_as_string();
    let captcha_img = captcha
        .apply_filter(Noise::new(0.5))
        .view(220, 80)
        .as_base64()
        .expect("captcha  create failed");

    (captcha_id, captcha_code, captcha_img)
}
