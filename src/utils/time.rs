use rbatis::rbdc::DateTime;
use std::time::SystemTime;

pub fn get_timestamp() -> u64 {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

pub static DATETIME_FORMAT: &str = "YYYY-MM-DD hh:mm:ss";

#[inline]
pub fn format_date(datetime: DateTime) -> String {
    datetime.format(DATETIME_FORMAT)
}

#[inline]
pub fn format_date_option(datetime: Option<DateTime>) -> Option<String> {
    datetime.map(format_date)
}
