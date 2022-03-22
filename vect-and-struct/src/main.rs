use colored::Colorize;
use std::io::{self, Write};

struct Pair {
    key: String,
    value: String,
}

fn main() {
    println!("📋 Let's create a list of key/value pairs!");

    let list_length: usize = loop {
        let mut desired_length = String::new();

        print!("How long do you want the list to be? (maximum 20 pairs) ");

        /*
            Note that stdout is frequently line-buffered by default so it may be necessary
            to use io::stdout().flush() to ensure the output is emitted immediately.
        */
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut desired_length)
            .expect("Failed to read line");

        let desired_length: usize = match desired_length.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a valid number", desired_length.red());
                0
            }
        };

        if desired_length == 0 {
            continue;
        } else {
            if desired_length > 20 {
                println!("The list cannot exceed {} elements!", "20".red());
                continue;
            } else {
                break desired_length;
            }
        }
    };

    let mut i: usize = 0;
    // Initialize an empty array of length 20 of string tuples
    let mut pairs = Vec::new();

    while i < list_length {
        let new_pair = ask_for_struct();
        pairs.push(new_pair);
        i += 1;
    }

    // Reset "i" to reuse variable to save memory :)
    i = 0;

    println!("Let's take a look at your key/value pairs!\n");

    while i < list_length {
        println!(
            "Key: {}\rValue: {}",
            pairs[i].key.green(),
            pairs[i].value.green()
        );
        i += 1;
    }
}

fn ask_for_struct() -> Pair {
    print!("Please enter the {} for this pair: ", "key".green());

    io::stdout().flush().unwrap();

    let mut key = String::new();
    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read line");

    let mut value = String::new();

    print!("Please enter the {} for this pair: ", "value".green());

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    println!("----");

    Pair { key, value }
}
