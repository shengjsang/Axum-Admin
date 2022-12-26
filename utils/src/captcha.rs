use crate::redis::{connect, set};
use anyhow::Result;
use captcha::filters::Noise;
use captcha::Captcha;
use rand::Rng;

pub async fn new() -> Result<()> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const CAPTCHA_ID_LEN: usize = 16;
    let mut rng = rand::thread_rng();

    let captcha_id: String = (0..CAPTCHA_ID_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    let mut captcha = Captcha::new();
    let captcha_code = captcha.add_chars(6).chars_as_string();
    let _captcha_img = captcha
        .apply_filter(Noise::new(0.5))
        .view(220, 80)
        .as_base64()
        .expect("captcha  create failed");
    let mut con = connect().await.unwrap();
    set(
        &mut con,
        captcha_id.as_str(),
        captcha_code.as_str(),
        60 * 15,
    )
    .await
    .unwrap();
    Ok(())
}
