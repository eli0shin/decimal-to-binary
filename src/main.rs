use std::env;
mod to_binary;
use to_binary::convert as to_binary;

fn main() {

    let args: Vec<String> =
        env::args().collect();

    match args.len() {
        1 => {
            println!("Please enter a valid i64 to convert");
        },
        _ => {
            match args[1].parse::<i64>() {
                Ok(n) => {
                    println!("{}", to_binary(n, String::from("")));
                },
                _ => {
                    println!("Please enter a valid i64 to convert");
                }
            }
        },
    }

}
