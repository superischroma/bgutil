//use std::io::Write;

use front::send_notification;

pub mod front;
pub mod itemtracker;
pub mod tray;

pub enum ThreadState
{
    Finished,
}

const VERSION: &str = "1.0";

fn main()
{
    let (tx, rx) = std::sync::mpsc::channel::<ThreadState>();
    itemtracker::start_process(&tx);
    tray::start_process(&tx);
    send_notification("Core", format!("Background Utility Modules v{} initialized", VERSION).as_str());
    loop
    {
        match rx.recv()
        {
            Ok(ThreadState::Finished) => break,
            _ => {},
        }
    }
    /*
    loop
    {
        println!("-- BUM (v{}) --", VERSION);
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