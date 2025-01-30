fn main() {
    let x = 5; // immutable

    println!("The value of x is: {x}");

    let x = x + 1; // could not re-assign unless re let

    println!("The value of x with one added is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x after the scope is: {x}");
}



// fn main() {
//     // println!("Hello, world!");
//     // const HELLO_WORLD: String = "Hello world";
//     // println!(HELLO_WORLD);

//     const ONE_HUNDRED: u32 = 10 * 10;
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = x * ONE_HUNDRED;
//     println!("The value of x is: {x}");
// }
