use std::{fs::File, io::Read};
use once_cell::sync::Lazy;
use crate::config::Configs;
mod config;


// 设置配置文件路径
const CFG_FILE: &str = "Config.toml";
// 在第一次访问的时候初始化值 静态的 之后可以一直使用
pub static CFG: Lazy<Configs> = Lazy::new(Configs::init);

impl Configs {
    pub fn init() -> Self {
        let mut file = match File::open(CFG_FILE) {
            Ok(f) => f,
            Err(e) => panic!("找不到配置文件：{}，错误信息：{}", CFG_FILE, e),
        };
        let mut config_info = String::new();
        // 读取内容
        file.read_to_string(&mut config_info).expect("读取配置文件内容错误");
        // toml -> Configs
        toml::from_str(&config_info).expect("解析配置文件错误")
    }
}
