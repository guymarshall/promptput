#![forbid(unsafe_code)]

use std::error::Error;
use std::io;
use std::str::FromStr;

fn get_user_input<T: FromStr>(prompt: &str) -> Result<T, Box<dyn Error>> {
    println!("{}", prompt);

    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input)?;

    let number: T = user_input
        .trim()
        .parse()
        .map_err(|_| "Please enter a valid number!")?;

    Ok(number)
}

pub fn input<T: FromStr>(prompt: &str) -> T {
    loop {
        match get_user_input(prompt) {
            Ok(count) => {
                return count;
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        };
    }
}
