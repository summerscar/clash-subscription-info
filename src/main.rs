use std::process::Command;
use chrono::{NaiveDateTime, Utc};
use regex::Regex;
use simple_user_input::get_input;
use std::env;

fn main() {
  let clash_url;

  let args: Vec<String> = env::args().collect();
  if args.len() <= 1 {
    clash_url = get_input("Please input a clash subscribe url...");
  } else {
    clash_url = String::from(&args[1]);
  }


  let output = Command::new("curl")
    .args(["-sLI","-X", "GET", "-H", "User-Agent: Clash" ])
    .arg(clash_url)
    .output()
    .expect("failed to execute process");


  let reg = Regex::new(r"download=(\d+); total=(\d+); expire=(\d+)").unwrap();
  let text = String::from_utf8_lossy(&output.stdout);

  if text.len() < 1 {
    eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    println!("Please input a valid clash subscribe url...");
    return;
  }

  for cap in reg.captures_iter(&text) {
    let used = cap[1].parse::<u64>().unwrap() / 1024 / 1024 / 1024;
    let total = cap[2].parse::<u64>().unwrap() / 1024 / 1024 / 1024;

    let expire = NaiveDateTime::from_timestamp(cap[3].parse::<i64>().unwrap(), 0);
    let expire_day = expire.format("%Y-%m-%d").to_string();
    let expire_left = expire.signed_duration_since(Utc::now().naive_utc()).num_days();

    println!("Download: {}G, Total: {}G, Left: {}days, Expire: {}", used, total, expire_left, expire_day);

    return
  }

  println!("Please input a valid clash subscribe url...");
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}
