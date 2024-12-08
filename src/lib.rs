use chrono::Local;
use colored::*;
use fern::Dispatch;
use std::sync::Once;

/// 用于初始化 CLogger。
///
/// 该函数会初始化 CLogger 并进行一些配置。需要通过 `log_file_path` 参数指定日志文件的保存位置。
///
/// # 示例
/// ```rust
/// use clogger::init_clogger;
///
/// init_clogger("/tmp/clogger_example.log"); // 将 '/tmp/clogger_example.log' 设置为日志文件的保存位置
/// ```
/// ```rust
/// use clogger::init_clogger;
///
/// init_clogger("/dev/null"); // 将 Unix 黑洞设置为日志文件的保存位置，这将不会实际写入任何日志
/// ```
///
/// # 参数
/// - `log_file_path`: 日志文件的保存位置。
pub fn init_clogger(log_file_path: &str) {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let base_config = Dispatch::new()
            .format(|out, message, record| {
                let timestamp = Local::now()
                    .format("%Y-%m-%d %H:%M:%S%.3f")
                    .to_string()
                    .cyan();
                let level = match record.level() {
                    log::Level::Info => "I".to_string().green(), // 普通日志为绿色
                    log::Level::Warn => "W".to_string().yellow(), // 警告日志为黄色
                    log::Level::Error => "E".to_string().red(),  // 错误日志为红色
                    log::Level::Debug => "D".to_string().blue(), // 调试日志为蓝色
                    log::Level::Trace => "T".to_string().purple(), // 追踪日志为紫色
                };
                out.finish(format_args!(
                    "({}) [{}] [{}] {}",
                    timestamp,
                    level,
                    record.target().magenta(),
                    message
                ))
            })
            .level(log::LevelFilter::Debug)
            .chain(std::io::stdout()) // 输出到终端
            .chain(fern::log_file(log_file_path).unwrap()); // 写入日志文件

        base_config.apply().unwrap();
        c_log!("CLogger 初始化完成 (ง •_•)ง");
    });
}

/// 用于输出和记录常规日志。
///
/// 该宏会将日志信息输出到终端并写入日志文件，日志的级别为 `Info`。可以通过 `$moudle` 参数指定模块名称。
/// 若未指定 `$module` 参数，将使用 `module_path!()` 自动获取模块名称。
///
/// # 示例
/// ```rust
/// use clogger::{init_clogger, c_log};
///
/// // 初始化 CLogger
/// init_clogger("/tmp/clogger_example.log");
/// // 输出日志
/// c_log!("example::moudle_name", "这是一条常规日志！(づ｡◕‿‿◕｡)づ");
/// ```
///
/// # 参数
/// - `$module` (可选): 模块名称。
/// - `$message`: 日志信息内容。
#[macro_export]
macro_rules! c_log {
    ($message:expr) => {
        {
            c_log!(module_path!(), $message);
        }
    };
    ($module:expr, $message:expr) => {
        {
            use log::info;
            use std::panic::Location;
            let location = Location::caller();
            info!(target: format!("{} ({}:{}^{})", $module, location.file(), location.line(), location.column()).as_str(), "{}", $message);
        }
    };
}

/// 用于输出和记录警告日志。
///
/// 该宏会将日志信息输出到终端并写入日志文件，日志的级别为 `Warn`。可以通过 `$moudle` 参数指定模块名称。
/// 若未指定 `$module` 参数，将使用 `module_path!()` 自动获取模块名称。
///
/// # 示例
/// ```rust
/// use clogger::{init_clogger, c_warn};
///
/// // 初始化 CLogger
/// init_clogger("/tmp/clogger_example.log");
/// // 输出日志
/// c_warn!("example::moudle_name", "这是一条警告日志！w(ﾟДﾟ)w");
/// ```
///
/// # 参数
/// - `$module` (可选): 模块名称。
/// - `$message`: 日志信息内容。
#[macro_export]
macro_rules! c_warn {
    ($message:expr) => {
        {
            c_warn!(module_path!(), $message);
        }
    };
    ($module:expr, $message:expr) => {
        {
            use log::warn;
            use colored::Colorize;
            use std::panic::Location;
            let location = Location::caller();
            warn!(target: format!("{} ({}:{}^{})", $module, location.file(), location.line(), location.column()).as_str(), "{}", $message.yellow());
        }
    };
}

/// 用于输出和记录错误日志。
///
/// 该宏会将日志信息输出到终端并写入日志文件，日志的级别为 `Error`。可以通过 `$moudle` 参数指定模块名称。
/// 若未指定 `$module` 参数，将使用 `module_path!()` 自动获取模块名称。
///
/// # 示例
/// ```rust
/// use clogger::{init_clogger, c_error};
///
/// // 初始化 CLogger
/// init_clogger("/tmp/clogger_example.log");
/// // 输出日志
/// c_error!("example::moudle_name", "这是一条错误日志！＞﹏＜");
/// ```
///
/// # 参数
/// - `$module` (可选): 模块名称。
/// - `$message`: 日志信息内容。
#[macro_export]
macro_rules! c_error {
    ($message:expr) => {
        {
            c_error!(module_path!(), $message);
        }
    };
    ($module:expr, $message:expr) => {
        {
            use log::error;
            use colored::Colorize;
            use std::panic::Location;
            let location = Location::caller();
            error!(target: format!("{} ({}:{}^{})", $module, location.file(), location.line(), location.column()).as_str(), "{}", $message.red());
        }
    };
}

/// 用于输出和记录调试日志。
///
/// 该宏会将日志信息输出到终端并写入日志文件，日志的级别为 `Debug`。可以通过 `$moudle` 参数指定模块名称。
/// 若未指定 `$module` 参数，将使用 `module_path!()` 自动获取模块名称。
///
/// # 示例
/// ```rust
/// use clogger::{init_clogger, c_debug};
///
/// // 初始化 CLogger
/// init_clogger("/tmp/clogger_example.log");
/// // 输出日志
/// c_debug!("example::moudle_name", "这是一条调试输出！(ง •_•)ง");
/// ```
///
/// # 参数
/// - `$module` (可选): 模块名称（用于在日志中标记日志来源）。
/// - `$message`: 日志信息内容。
#[macro_export]
macro_rules! c_debug {
    ($message:expr) => {
        {
            c_debug!(module_path!(), $message);
        }
    };
    ($module:expr, $message:expr) => {
        {
            use log::debug;
            use std::panic::Location;
            let location = Location::caller();
            debug!(target: format!("{} ({}:{}^{})", $module, location.file(), location.line(), location.column()).as_str(), "{}", $message);
        }
    };
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::*;

    #[test]
    fn test_logging() {
        init_clogger("/tmp/test_clogger.log");

        let x = 42;

        c_log!(
            "clogger::tests::test_logging",
            "这是一条常规日志！(づ｡◕‿‿◕｡)づ"
        );
        c_log!("这也是一条常规日志！(づ｡◕‿‿◕｡)づ");
        c_warn!("clogger::tests::test_logging", "这是一条警告日志！w(ﾟДﾟ)w");
        c_warn!("这也是一条警告日志！w(ﾟДﾟ)w");
        c_error!("clogger::tests::test_logging", "这是一条错误日志！＞﹏＜");
        c_error!("这也是一条错误日志！＞﹏＜");
        c_debug!("clogger::tests::test_logging", "这是一条调试输出！(ง •_•)ง");
        c_debug!("这也是一条调试输出！(ง •_•)ง");
        c_debug!(
            "clogger::tests::test_logging()",
            format!("(format! Test) 变量 x 的内容为: {}", x)
        );
    }

    #[test]
    fn perf_test() {
        init_clogger("/tmp/test_clogger.log");

        for i in 1..=1_000_000 {
            c_log!("tests::perf_test()", format!("性能测试 ing... (x{i})"));
        }

        c_log!(
            "tests::perf_test()",
            "开始清空性能测试残留 ('echo > /tmp/test_clogger.log')"
        );
        Command::new("sh")
            .arg("-c")
            .arg("echo > /tmp/test_clogger.log")
            .output()
            .expect("清空 /tmp/test_clogger.log 时出现错误");
    }
}
