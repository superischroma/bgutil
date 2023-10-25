use std::{env, fs::{File, OpenOptions}, path::Path};

use notify_rust::Notification;

pub fn send_notification(process: &str, message: &str)
{
    Notification::new()
        .summary(process)
        .body(message)
        .icon(format!("{}\\icon.ico", env::current_exe().unwrap().parent().unwrap().to_str().unwrap()).as_str())
        .appname("bgutil")
        .show().unwrap();
    //OpenOptions::new().truncate(false).write(true).open(Path::new(""));
}