use ferris_says::say;
use std::io;
use std::io::{stdout, BufWriter};


fn main() {
    
    println!("Please enter your name:");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Hello, {}!", name);

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}