use log::LevelFilter;

fn logger_init() {
    let mut builder = colog::default_builder();
    builder.filter_level(LevelFilter::Trace);
    builder.init();
    log::info!("Logger initialized");
}

pub fn init() {
    logger_init();
}