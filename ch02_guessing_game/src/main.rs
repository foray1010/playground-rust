extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input;
}

fn main() {
    let max = 100;
    let min = 1;
    let show_answer = false;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(min, max + 1);
    if show_answer {
        println!("The secret number is: {}", secret_number);
    }

    loop {
        println!("Please input your guess.");

        let guessed_number: f64 = match get_input().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let guessed_number = guessed_number as i32;
        println!("You guessed: {}", guessed_number);

        if guessed_number > max {
            println!("Excess upper limit!");
            continue;
        }

        if guessed_number < min {
            println!("Excess lower limit!");
            continue;
        }

        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
