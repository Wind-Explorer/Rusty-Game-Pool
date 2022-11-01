use std::{io, process::exit};
use rand::Rng;
use std::cmp::Ordering;

pub fn forklift() {
    let sn = rand::thread_rng().gen_range(1..=100);
    forklift_game(false, sn);
}

fn forklift_game(repeat: bool, secret_number: i32) {
    if !repeat {
        println!("Forklift goes up and down.");
        println!("What do you think:");
    } else {
        println!("\nGuess:");
    }

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to see what you typed ;-;");
    match user_input.trim().parse::<i32>() {
        Ok(n) => compare_number(n, secret_number),
        Err(_) => not_number(secret_number),
    }
}

fn compare_number(num: i32, secret: i32) {
    match num.cmp(&secret) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => user_win(),
    }
    forklift_game(true, secret);
}

fn not_number(secret: i32) {
    println!("That not number bru");
    forklift_game(true, secret);
}

fn user_win() {
    println!("Right about there!\nGood job :)");
    exit(0);
}
