use serde::Serialize;

/// 验证码响应
#[derive(Debug, Serialize, Default)]
pub struct Captcha {
    pub captcha_id: Option<String>,
    pub captcha_image: Option<String>,
}
