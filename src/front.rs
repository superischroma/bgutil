use std::{fs::OpenOptions, path::Path, io::Write, time::SystemTime};

use chrono::{DateTime, Local};
use notify_rust::Notification;

pub fn send_notification(process: &str, message: &str)
{
    Notification::new()
        .summary(process)
        .body(message)
        .icon(format!("{}\\icon.ico", std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap()).as_str())
        .appname("bum")
        .show().unwrap();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(Path::new(format!("{}\\notifications.log", std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap()).as_str())).unwrap();
    let dt = DateTime::<Local>::from(SystemTime::now()).format("%F | %r").to_string();
    file.write_all(format!("[{} | {}]: {}\n", process, dt, message).as_bytes()).unwrap();
}