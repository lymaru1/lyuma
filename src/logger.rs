use flexi_logger::{Logger, FileSpec, Age};
use log::{LevelFilter, debug, info, error};

pub fn init_logger() {
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory("logs"))
        .rotate(
            flexi_logger::Criterion::Age(Age::Day),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(7),
        )
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
}

// 트레이트 정의
pub trait Loggable {
    fn log_function_call(&self, function_name: &str);
}

// 트레이트 구현
impl<T> Loggable for T {
    fn log_function_call(&self, function_name: &str) {
        info!("Function called: {}", function_name);
    }
}

pub fn log_debug(message: &str) {
    debug!("{}", message);
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}