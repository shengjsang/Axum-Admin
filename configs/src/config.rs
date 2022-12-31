use serde::Deserialize;

/// 配置文件
#[derive(Debug, Deserialize)]
pub struct Configs {
    /// 程序配置
    pub server: Server,
    /// 日志配置
    pub log: Log,
    pub database: Database,
    pub redis: Redis,
    pub captcha: Captcha,
    pub jwt: JWT,
}

/// server 配置文件
#[derive(Debug, Deserialize)]
pub struct Server {
    /// 服务器名称
    pub name: String,
    /// 服务器(IP地址:端口)
    /// `0.0.0.0:3000`
    pub address: String,
}

/// 日志配置
#[derive(Debug, Deserialize)]
pub struct Log {
    /// `log_level` 日志输出等级
    pub log_level: String,
    /// `dir` 日志输出文件夹
    pub dir: String,
    /// `prefix` 日志输出文件前缀名
    pub prefix: String,
}

/// 数据库配置
#[derive(Debug, Deserialize)]
pub struct Database {
    /// `url` 数据库连接
    pub url: String,
}

/// Redis配置
#[derive(Debug, Deserialize)]
pub struct Redis {
    /// `url` redis连接
    pub url: String,
}

/// 验证码配置
#[derive(Debug, Deserialize)]
pub struct Captcha {
    /// [`length`] 验证码长度
    pub length: u32,
    pub noise: f32,
    pub width: u32,
    pub height: u32,
}

/// Redis配置
#[derive(Debug, Deserialize)]
pub struct JWT {
    /// `url` redis连接
    pub secret: String,
    pub exp: i64,
}
