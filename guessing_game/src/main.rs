use rand::Rng; // rand library used to generate random numbers
use std::cmp::Ordering;
use std::io; // io library used to take input from user and for output operations // cmp library used to compare two values
             // cargo doc --open to open the documentation of the libraries used in the project

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Generate a random number between 1 and 100
                                                               // 1..=100 is similiar to start..=end, which is a range that includes the start and end numbers.

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Mutable(Changeable) variable to store the user input
                                       // :: is used to callan associated function of a type, in this case, new is an associated function of String type.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Read the input from user and store it in the mutable variable.
                                            // & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
                                            // expect is a method of Result type, which is used to handle errors. If the Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Ok is an enum variant that indicates that the parse operation was successful. num is the number that was parsed from the string.
            Err(_) => continue, // _ is a catchall value. In this case, we’re saying that we want to match all Err values, no matter what information they have inside them. The continue keyword tells the program to go to the next iteration of the loop.
        }; // Convert the user input to a number
           // Shadowing lets us reuse the variable. trim() method eliminates the whitespace from the string. parse() method parses the string into a number. u32 is an unsigned 32-bit integer.

        println!("You guessed: {}", guess); // Print the user input

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    // match expression is made up of arms. An arm consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. The match construct and the arms are separated by curly braces.
}
