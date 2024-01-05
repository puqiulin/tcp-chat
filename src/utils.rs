use chrono::Local;

pub fn get_local_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
