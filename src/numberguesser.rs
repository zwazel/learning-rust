/// Guess the number game
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
pub mod numberguesser {
    use std::cmp::{Ordering, min};
    use std::io;

    use rand::Rng;

    pub fn guess_the_number() {
        println!("Guess the number!");

        let mut tries = 0;
        let mut min_range = 1;
        let mut max_range = 100;
        let secret_number = rand::thread_rng().gen_range(min_range-1..max_range + 1);

        // println!("The secret number is: {}", secret_number);

        loop {
            println!("Please input your guess. ({}-{})", min_range, max_range);

            // - "let mut" -> declare a mutable variable, normally variables are immutable
            // - "::new()" -> is like a static function of the type String, so with "::" we say it is an associated function of the String type.
            let mut guess = String::new();

            // - The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
            io::stdin()
                .read_line(&mut guess) // "&" tells rust that it is a reference, with "mut" we say it is mutable. by default, references ir immutable. references are a safe way to access a variable without always copying it in the memory.
                // read line returns a Result, results are enums. for result, the variants are "Ok" or "Err". the "Ok" varian indicates the operation was successful and contains the generated value, "Err" is not successful and contains information about how or why the operation failed
                .expect("Failed to read line"); // "expect" is a method of the Result type. it gets called when Result is a variant of Err. This is not like "try, catch". the program will still crash

            // even though we already have a variable with the name "guess" we can 'shadow' the previous value of "guess" with a new one
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => { // The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them.
                    println!("Please type a number!");
                    continue;
                }
            };

            println!("You guessed: {}", guess);

            tries += 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Too small!");
                    if guess > min_range {
                        min_range = guess;
                    }
                },
                Ordering::Greater => {
                    println!("Too big!");
                    if guess < max_range {
                        max_range = guess;
                    }
                },
                Ordering::Equal => {
                    println!("You win!");
                    println!("It took you {} tries to win!", tries);
                    break;
                }
            }

            println!("Amount tries: {}", tries);
        }
    }
}