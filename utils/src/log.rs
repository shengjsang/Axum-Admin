use chrono::Local;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::{Compact, Format, Writer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, fmt, Registry};
use tracing_subscriber::fmt::time::FormatTime;
use configs::CFG;

pub fn init() -> WorkerGuard {

    // 设置日志为每天轮换 - 存放目录 - 日志文件名前缀
    let file_appender = tracing_appender::rolling::daily(&CFG.log.dir, &CFG.log.prefix);

    let (non_blocking, file_guard) = tracing_appender::non_blocking(file_appender);

    // 从Config.toml 读取设置的日志显示级别
    let log_level = get_level();

    let logger = Registry::default()
        .with(EnvFilter::from_default_env().add_directive(log_level.into()))
        .with(fmt::Layer::default().with_writer(non_blocking).event_format(formats()));


    // 收集日志设置全局默认值 返回是否初始化成功
    tracing::subscriber::set_global_default(logger).unwrap();

    // 必须返回guard到main()函数 不然日志文件配置失败
    file_guard
}

// 设置日志打印的格式
pub fn formats() -> Format<Compact, LocalTimer> {
    fmt::format()
        .with_ansi(false)
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_timer(LocalTimer)
        .compact()
}


//  读取配置文件中的日志记录级别
pub fn get_level() -> Level {
    match CFG.log.log_level.as_str() {
        "TRACE" => Level::TRACE,
        "DEBUG" => Level::DEBUG,
        "INFO" => Level::INFO,
        "WARN" => Level::WARN,
        "ERROR" => Level::ERROR,
        _ => Level::INFO,
    }
}

// 设置日志日期格式
pub struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT %T"))
    }
}
