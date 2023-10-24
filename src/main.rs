pub mod front;
pub mod itemtracker;

const VERSION: &str = "1.0";

fn main()
{
    itemtracker::start_process();
    loop
    {
        println!("-- bgutil (v{}) --", VERSION);
        println!("[1] Edit Item Tracker");
        println!("[2] End Program");
        print!(">> ");
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input).expect("failed to read input from user");
        input.retain(|c| c != '\r' && c != '\n');
        match input.as_str() {
            "1" => itemtracker::edit(),
            "2" => break,
            _ => println!("unexpected input: {}", input),
        };
    }
}
