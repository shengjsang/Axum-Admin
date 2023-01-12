use crate::redis::{connect, set};
use anyhow::Result;
use captcha::filters::Noise;
use captcha::Captcha;
use configs::CFG;
use crate::rand::CharSetKind::NumberAndLetter;
use crate::rand::Custom;

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
    let captcha_id = Custom::new(12,NumberAndLetter).generate();

    let mut captcha = Captcha::new();
    let captcha_code = captcha.add_chars(CFG.captcha.length).chars_as_string();
    let captcha_img = captcha
        .apply_filter(Noise::new(CFG.captcha.noise))
        .view(CFG.captcha.width, CFG.captcha.height)
        .as_base64()
        .expect("captcha  create failed");

    (captcha_id, captcha_code, captcha_img)
}
