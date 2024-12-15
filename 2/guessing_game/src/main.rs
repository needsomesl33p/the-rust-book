use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    //  inclusive declaration
    //  not-inclusive declaration: gen_range(1..101)
    let secret_number = rand::thread_rng().gen_range(1..=100); // by default variables ar inmutables

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // creating a new mutable string -> value can be assigned multiple times

        io::stdin()
            .read_line(&mut guess) // & -> it indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
            .expect("Failed to read line"); // expect method is able to catch the Err Result variant and exit the program with an err message

        // variable shadowing -> reusing the same variable name
        // parse() requires a type, which is u32 - unsigned 32-bit integer (non-negative)
        let guess: u32 = match guess.trim().parse() {
            // OK and Err are Result variants
            // Result is an Enum type
            // Parse return an OK or an Err and the match keyword decides to what happends next
            Ok(num) => num, // parse() returns the number and if everthing is OK guess will have the value of num
            Err(_) => continue, // continue meaning do not throw error
                             // "_" -> means all error is catched
        };

        println!("You guessed: {guess}"); // String subsitution

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // we can do multiple things in the curly brackets
                println!("You win!");
                break; // Stop the loop
            }
        }
    }
}
