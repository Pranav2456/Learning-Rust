enum IpAddrKind {
    V4,
    V6,
} // This is a now a custom data type.

fn main() {
    // Enums - Enums allow you to define a type by enumerating its possible variants.
    // Enums are useful when you wanto to say that a value could be one of a few different types.

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    } // We can also define enum with data associated with each variant.

    // One can put any kind of data inside an enum variant: strings, numeric types, structs, or even other enums.

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127,0,0,1);

    let loopback = IpAddr::V6(String::from("::1"));

    route(IpAddrKind::V4); // Now we can call this function with either of the values.

    // Below is a good example of Enum
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // As we are able to define methods on structs, we can also define methods on enums.

    impl Message {
        fn call(&self) {
            // Whatever bla bla bla
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option Enum - Option is a type that represents a value that could be something or nothing.
    // enum Option<T> {
    //     None,
    //     Some(T),
    // } // Some(T) and None are variants of Option.
    // Here <T> is a generic type parameter. This means the Some variant of the Option enum can hold one piece of data of any type.
    // But that each concrete instance of Option can only hold values of one type.

    let some_number = Some(5); // Here the type of some_number is Option<i32>
    let some_string = Some("a string"); // Here the type of some_string is Option<&str>

    let absent_number: Option<i32> = Option::None; // Here Rust requires us to tell it what type of Option<T> we have, because it can't infer the type that the Some variant will hold by looking only at a None value.

    // Eliminating the risk of incorrectly assuming a not-null helps you to be more confident in your code. In order to have a value that can be possibly null, you must explicitly opt in by making the type of that value Option<T>.
    // Then when we use that value, we are required to explicitly handle the case when the value is null.
    // Everywhere you have a value with a type that is not an Option<T>, you can safely assume that the value is not null.
    // Delibrate design choice in Rust to limit null's pervasiveness and increase the safety of Rust code.

    // THe match control flow operator is used to compare a value against a series of patterns and then execute code based on which pattern matches.
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky Penny!");
                1
            }
            Coin::Nickel => {
                println!("Five Cents!");
                5
            }
            Coin::Dime => {
                println!("Ten Cents!");
                10
            }
            Coin::Quarter(state) => {
                println!("State Quarter from {state:?}!");
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));

    // Matching with Option<T> Example
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six: {:?}, None: {:?}", six, none);

    // Matches are exhaustive - meaning that you must exhaust every last possibility in order for the code to be valid.

    // Catch-all patterns and the _ placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
     // _ => reroll(), // The _ pattern will match any value, so it's often used in the last match arm to catch any other value that isn't specified before it.
        _ => (), // This is a unit value, so nothing will happen.
    } //  The other pattern will match any value that isn't specifically handled by the other patterns.
    // Note that the other value is always the last pattern in the match expression.\

    // Rust also has a pattern we can use when we want to catch-all but we don't want to do anything with the value.
    // _ is a special pattern in Rust that will match any value and does not bind to the value.

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}

    // If let - A shorter way to write a match that only cares about one case.
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The Maximum is configured to be {max}");
    } else {
        println!("No maximum configured");
    } // This is the same as writing a match that only matches one case and ignores the rest.

    // If let can also have an else block, which is the same as the _ case in a match.
}

fn route(ip_kind: IpAddrKind) {}