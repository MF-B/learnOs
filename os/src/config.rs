// use log::Level;

// // 存储系统配置
// pub struct Config {
//     pub log_level: Level,
//     pub log_en: bool,
// }

// impl Config {
//     // 从环境变量或类似机制解析配置
//     pub fn parse_from_cmdline() -> Self {
//         #[cfg(feature = "log_trace")]
//         let level = Level::Trace;

//         #[cfg(feature = "log_debug")]
//         let level = Level::Debug;
        
//         #[cfg(feature = "log_info")]
//         let level = Level::Info;
        
//         #[cfg(feature = "log_warn")]
//         let level = Level::Warn;
        
//         #[cfg(feature = "log_error")]
//         let level = Level::Error;

//         #[cfg(feature = "log_disable")]
//         let log_en = false; 

//         #[cfg(not(feature = "log_disable"))]
//         let log_en = true; 
        
//         #[cfg(not(any(feature = "log_trace", feature = "log_debug", feature = "log_info", feature = "log_warn", feature = "log_error")))]
//         let level = Level::Info;
        
//         Self {
//             log_level: level,
//             log_en
//         }
//     }
// }