use android_logger::{Config, FilterBuilder};

use log::{Level, LevelFilter};

pub struct Logger {}

impl Logger {
    pub fn init() {
        #[cfg(debug_assertions)]
        let log_filter = FilterBuilder::new()
            .filter_level(LevelFilter::Trace)
            .build();
        #[cfg(not(debug_assertions))]
        let log_filter = FilterBuilder::new().filter_level(LevelFilter::Off).build();
        android_logger::init_once(
            Config::default()
                .with_filter(log_filter)
                .with_min_level(Level::Trace)
                .with_tag("rust_log"),
        );
    }
}
