use log::LevelFilter;
use oslog::OsLogger;

pub struct Logger;

impl Logger {
    pub fn init() {
        #[cfg(debug_assertions)]
        let log_filter = LevelFilter::Trace;
        #[cfg(not(debug_assertions))]
        let log_filter = LevelFilter::Off;

        let _ = OsLogger::new("com.minyung.rust")
            .level_filter(log_filter)
            .category_level_filter("category", LevelFilter::Trace)
            .init();
    }
}
