
pub fn init_logger() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
}