use std::io;
use rand::Rng; // Rng is a trait of rand, defines methods that random number generators implement. This trait must be in scope for us to use those methods (in this case below, the gen_range method)

fn main() {
    println!("Guess the number!");

    // We call the
    //    rand::thread_rng function
    //    that gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system
    // Then we call the
    //    gen_range method
    //    on the random number generator. This method is defined by the Rng trait that we brought into scope with the use rand::Rng; statement.

    let the_secret_number = rand::thread_rng().gen_range(1..=100); // range expression here is inclusive on the lower and upper bounds 1 to 100.

    println!("The secret number is {the_secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}