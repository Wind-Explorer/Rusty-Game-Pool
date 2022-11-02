use std::io;
mod forklift;

fn main() {
    println!("Welcome to Game Pool!");
    println!("We have:
1 -> Forklift
0 -> Exit Game Pool
");
    println!("Which game would you like to play today?");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to capture input.");
    match user_input.trim().parse::<i32>() {
        Ok(n) => read_input(n),
        Err(_) => not_number(),
    }
}

fn read_input(input: i32) {
    match input {
        0 => std::process::exit(0),
        1 => crate::forklift::forklift(),
        _ => not_number(),
    }
}

fn not_number() {
    println!("Not an option! Quitting...");
    std::process::exit(1);
}