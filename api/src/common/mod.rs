use model::common::response::Captcha;
use model::response::Res;
pub mod jwt;

pub async fn show_captcha() -> Res<Captcha> {
    let res = utils::captcha::new().await;
    match res {
        Ok((captcha_id, captcha_image)) => Res::ok_with_data(Captcha {
            captcha_id: Some(captcha_id),
            captcha_image: Some(captcha_image),
        }),
        Err(e) => Res::error_with_msg(500, e.to_string()),
    }
}

pub async fn test() -> Res<Captcha> {
    Res::ok_with_msg("test".to_string())
}
