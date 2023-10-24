use std::thread;

use json::JsonValue;
use futures::executor::block_on;

fn req_ah(page: u32) -> JsonValue
{
    let url = format!("https://api.hypixel.net/skyblock/auctions?page={}", page);
    let future_get = reqwest::get(url);
    let result = block_on(future_get);
    if result.is_err()
    {
        ()
    }
    let response = result.unwrap();
    let text = block_on(response.text()).unwrap();
    json::parse(text.as_str()).unwrap()
}

fn run()
{

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