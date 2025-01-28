use rand::Rng; // Rng is a trait of rand, defines methods that random number generators implement. This trait must be in scope for us to use those methods (in this case below, the gen_range method)
use std::cmp::Ordering;
use std::io; // Ordering is an enum with variants Less, Greater, and Equal. We’ll use these variants to compare the player’s guess to the secret number.

fn main() {
    println!("Guess the number!");

    // We call the
    //    rand::thread_rng function
    //    that gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system
    // Then we call the
    //    gen_range method
    //    on the random number generator.
    //  The gen_range method is defined by the Rng trait that we brought into scope with the use rand::Rng; statement.

    let the_secret_number = rand::thread_rng().gen_range(1..=100); // range expression here is inclusive on the lower and upper bounds 1 to 100.

    println!("The secret number is {the_secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // &mut guess is a reference to the mutable guess variable, which allows read_line to modify the guess variable
            .expect("Failed to read line");

        // since taking stdin, substitue guess outside the "{}" of println!
        println!("You guessed (guess is a string): {}", guess);

        // parse the string input into a unsigned 32-bit integer, raise an error if it can't be parsed into a u32
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // Rust allows us to shadow the previous value of guess with a new one.
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example.
        // For now, know that this feature is often used when you want to convert a value from one type to another type.

        // The parse method on strings converts a string to another type. We need to tell Rust the exact number type we want by using let guess: u32. It’s a good default choice for a small positive number.
        // Additionally, the u32 annotation in this example and the comparison with secret_number means Rust will infer that secret_number should be a u32 as well. So now the comparison will be between two values of the same type!

        println!("You guessed (guess is now a u32): {}", guess);

        match guess.cmp(&the_secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
