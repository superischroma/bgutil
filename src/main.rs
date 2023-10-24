use std::env;

use notify_rust::Notification;

fn main()
{
    println!("{}", format!("{}\\icon.ico", env::current_exe().unwrap().parent().unwrap().to_str().unwrap()).as_str());
    Notification::new()
        .summary("Item Tracker")
        .body("blehhh")
        .icon(format!("{}\\icon.ico", env::current_exe().unwrap().parent().unwrap().to_str().unwrap()).as_str())
        .appname("bgutil")
        .show().unwrap();
}
