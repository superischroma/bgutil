//use std::io::Write;

use front::send_notification;
use tray_item::{TrayItem, IconSource};

pub mod front;
pub mod itemtracker;

const VERSION: &str = "1.0";

fn main()
{
    itemtracker::start_process();
    init_tray();
    send_notification("Core", format!("bgutil v{} initialized", VERSION).as_str());
    /*
    loop
    {
        println!("-- bgutil (v{}) --", VERSION);
        println!("[1] Edit Item Tracker");
        println!("[2] End Program");
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input).expect("failed to read input from user");
        input.retain(|c| c != '\r' && c != '\n');
        match input.as_str() {
            "1" => itemtracker::edit(),
            "2" => break,
            _ => println!("unexpected input: {}", input),
        };
    }
    */
}

fn init_tray()
{
    let icon_path = format!("{}\\icon.ico", std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap()).as_str();
    let mut tray = TrayItem::new("bgutil", IconSource::Resource(()));
}
