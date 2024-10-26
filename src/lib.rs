use simple_logger::SimpleLogger;

pub mod player;
pub mod region;

/** Initialises logging for the library */
pub fn init() {
    SimpleLogger::new().init().unwrap();
}
