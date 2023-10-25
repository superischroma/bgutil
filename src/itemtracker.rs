use chrono::{DateTime, Utc, TimeZone, FixedOffset, Local};
use std::{thread, io::Write, time::{SystemTime, Duration}, alloc::System};

use json::JsonValue;
use futures::executor::block_on;

use crate::front;

fn req_ah(page: u32) -> Option<JsonValue>
{
    let url = format!("https://api.hypixel.net/skyblock/auctions?page={}", page);
    let future_get = reqwest::get(url);
    let result = block_on(future_get);
    if result.is_err()
    {
        ()
    }
    let response = result.unwrap();
    if !response.status().is_success()
    {
        ()
    }
    let text = block_on(response.text()).unwrap();
    Option::Some(json::parse(text.as_str()).unwrap())
}

fn wait(secs: u64)
{
    let t = SystemTime::now();
    let dur_secs = Duration::from_secs(secs);
    while SystemTime::now() < t + dur_secs {
        let time_diff = t + dur_secs - SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let next_update = SystemTime::now() + dur_secs;
        let dt_time_diff = DateTime::<Utc>::from(time_diff);
        let ts_time_diff = dt_time_diff.format("%l:%M:%S %p").to_string();
        let dt_next_update = DateTime::<Local>::from(next_update);
        let ts_next_update = dt_next_update.format("%l:%M:%S %p").to_string();
        //println!("{} left until next update ({})", ts_time_diff, ts_next_update);
        thread::sleep(Duration::from_secs(10));
    }
    thread::sleep(Duration::from_secs(5));
}

fn run()
{
    loop
    {
        let test = req_ah(0);
        if test.is_none()
        {
            front::send_notification("Item Tracker", "Hypixel API down");
        }
    }
}

pub fn start_process()
{
    thread::spawn(run);
}

pub fn edit()
{
    loop
    {
        println!("-- Edit Item Tracker --");
        println!("[1] Return");
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input).expect("failed to read input from user");
        input.retain(|c| c != '\r' && c != '\n');
        match input.as_str() {
            "1" => {
                break;
            },
            _ => {
                println!("unexpected input: {}", input)
            }
        };
    }
}