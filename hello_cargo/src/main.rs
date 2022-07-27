// The io library comes from the standard library, known as std:
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // In Rust, variables are immutable by default, meaning once we give the variable a value, the value won't change.
    // To make a variable mutable, we add `mut` before the variable name.
    // The `::` syntax in the `::new` line indicates that `new` is an associated function of the `String` type. An associated function is a function that’s implemented on a type, in this case `String`.
    let mut guess = String::new();

    io::stdin()
        // variables, references are immutable by default. Hence, you need to write `&mut guess` rather than `&guess` to make it mutable.
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Rust allows us to shadow the previous value of `guess` with a new one.
    // Remember that `parse` returns a `Result` type and `Result` is an enum that has the variants `Ok` and `Err`. We’re using a `match` expression here, as we did with the `Ordering` result of the `cmp` method.
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    loop {
        // The `{}` set of curly brackets is a placeholder
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break; 
            }
        }
    }
}

