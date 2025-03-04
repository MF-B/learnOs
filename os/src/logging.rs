// use log::{Level, Record, Metadata};
// use core::fmt;


// // LOG启用
// static mut LOG_EN:bool = true; 
// // 全局LOG等级
// static mut LOG_LEVEL: Level = Level::Info;

// // 设置LOG等级
// pub fn set_max_level(level:Level){
//     unsafe { LOG_LEVEL = level };
// }
// pub fn set_log_en(log_en:bool){
//     unsafe { LOG_EN = log_en };
// }

// // LOG等级初始化
// pub fn init(level: Level,log_en: bool) {
//     set_max_level(level);
//     set_log_en(log_en);
// }

// // 检查日志等级决定是否输出
// pub fn enabled(metadata: &Metadata) -> bool{
//     if !unsafe { LOG_EN } {return false};
//     metadata.level() <= unsafe { LOG_LEVEL }
// }

// // 包装日志消息为一个Record
// pub fn get_record(args: fmt::Arguments,level: Level) -> Record {
//     // 创建并返回一个新的Record实例
//     Record::builder()
//         .args(args)
//         .level(level)
//         .target(module_path!())
//         .file(Some(file!()))
//         .line(Some(line!()))
//         .build()
// }

// // 日志宏调用的log函数
// pub fn log(record: &Record){
//     if enabled(record.metadata()) {
//         let level_color = match record.level() {
//             Level::Error => "\x1b[31m", // 红色
//             Level::Warn => "\x1b[93m",  // 亮黄色
//             Level::Info => "\x1b[34m",  // 蓝色
//             Level::Debug => "\x1b[32m", // 绿色
//             Level::Trace => "\x1b[90m", // 灰色
//         };
//         println!("{}[{}] {}\x1b[0m", level_color, record.level(), record.args());
//     }
// }


// // 宏定义
// #[macro_export]
// macro_rules! error {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::logging::log(&$crate::logging::get_record(format_args!($fmt $(, $($arg)+)?),log::Level::Error));
//     }
// }

// #[macro_export]
// macro_rules! warn {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::logging::log(&$crate::logging::get_record(format_args!($fmt $(, $($arg)+)?),log::Level::Warn));
//     }
// }

// #[macro_export]
// macro_rules! info {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::logging::log(&$crate::logging::get_record(format_args!($fmt $(, $($arg)+)?),log::Level::Info));
//         //$crate::console::print(format_args!($fmt $(, $($arg)+)?));
//     }
// }

// #[macro_export]
// macro_rules! debug {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::logging::log(&$crate::logging::get_record(format_args!($fmt $(, $($arg)+)?),log::Level::Debug));
//     }
// }

// #[macro_export]
// macro_rules! trace {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::logging::log(&$crate::logging::get_record(format_args!($fmt $(, $($arg)+)?),log::Level::Trace));
//     }
// }



/*！

本模块利用 log crate 为你提供了日志功能，使用方式见 main.rs.

*/

use log::{self, Level, LevelFilter, Log, Metadata, Record};

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };
        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }
    fn flush(&self) {}
}

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Info,
    });
}
