mod logger;
pub use logger::init_logger;

use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use tracing::info;

fn init_config() -> Config {
    // let config_file = args().find(|e| e.starts_with("--config="))
    //     .map(|e| e.split_at("--config=".len()).1.to_string())
    //     .unwrap_or("config/application.yaml".to_string());
    let config_file = env::var("CONFIG_FILE").unwrap_or("config/application.yaml".to_string());
    println!(
        "config_file: {}",
        config_file
    );
    #[cfg(not(debug_assertions))]
    {
        let path = std::env::current_exe().unwrap().parent().unwrap().join("");
        // println!("current_exe: {:?}", path);
        std::env::set_current_dir(path).unwrap();
    }
    info!(
        "current_dir: {:?}, config_file: {}",
        std::env::current_dir().unwrap(),
        config_file
    );
    let file = std::fs::File::open(config_file).expect("failed to open config file");
    let config: Config = serde_yaml::from_reader(file).expect("failed to parse file");
    // let errors = init_error();
    // config.errors = errors.errors;
    config
}

// fn init_error() -> ErrorConfig {
//     let config_file = "config/errors.json";
//     let file = std::fs::read_to_string(config_file).unwrap_or_default();
//     sonic_rs::from_str(file.as_str()).expect("failed to parse file")
// }

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub addr: String,
    pub cache_type: String,
    pub jwt_secret: String,
    pub jwt_exp: u64,
    pub jwt_refresh_token: u64,
    pub white_list_api: Vec<String>,
    pub login_fail_retry: u64,
    pub login_fail_retry_wait_sec: u64,
    pub trash_recycle_days: u64,
    pub datetime_format: String,
    // pub log: LogConfig,
    pub redis_url: String,
    pub db: DBConfig,
    #[serde(skip)]
    pub errors: HashMap<String, String>,
    pub default_password: String,
}

impl Config {
    pub fn new() -> Self {
        init_config()
    }
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub file_path: String,
    pub file_name: String,
    pub to_file: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file_path: "logs".to_string(),
            file_name: "app.log".to_string(),
            to_file: false,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct DBConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u32,
}

#[derive(Debug, Default, Deserialize)]
pub struct ErrorConfig {
    pub errors: HashMap<String, String>,
}

#[allow(clippy::len_zero)]
#[test]
fn load_config_test() {
    let config = Config::new();
    println!("{:#?}", config);
    assert!(config.addr.len() > 0);
}
