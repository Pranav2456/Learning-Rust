// An attribute to hide warnings of unused code.
#![allow(dead_code)]

fn main() {
    println!("Hello, world!");

    // Line Comments.
    /* Block 
    Comments */

    // In general the '{}' will be replaced by any arguments. These will be stringified
    println!("{} days", 31);
    // Positional arguments
    println!("{0}, this is {1}, this is {0}", "Alice", "Bob");
    // Named arguments
    println!("{Name} {Introduction} {General}", Name= "Bob", Introduction= "This is Bob", General = "He loves cars");
    // Different formattings
    println!("Base 10:   {}", 89076);
    println!("Binary: {:b}", 89076);
    println!("HexaDecimal: {:x}", 89076);
    println!("Octal: {:o}", 89076);

    let pi: f32 = 3.141592;
    println!("Pi: {:.6}", pi);

    let logical: bool = true;

    let a_float: f64 = 1.0;

    let default_float = 3.0; // f64

    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    let mut mutable = 12; // Mutable i32
    mutable = 21;

    //mutable = true; // Error: expected integer, found bool

    let mutable = true; // Variable can be overwritten with shadowing

    // Tuples
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    println!("Long tuple first value: {}",  long_tuple.0);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("Tuple of tuples: {:?}", tuple_of_tuples);

     // But long Tuples (more than 12 elements) cannot be printed.
     let pair = (1, true);
     println!("Pair is {:?}", pair);

     // one element tuple
        let one_element_tuple = (5u32,);

    // Destructured tuples
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    let tup: (bool, u32, f64) = (true, 42, 6.12);
    println!("{}", tup.0);

    // Arrays and Slices
    // Fixed size array
    let xs: [i32; 5] = [1,2,3,4,5];
    println!("Array element {}", xs[3]);

    // All elements of an array can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // "len" returns the count of elements in the array
    println!("First element of the array: {}", xs.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&xs));

    // Slices
    let empty_array: [u32; 0] = [];

    // Store a reference to an unsized type, because it is not possible to store an unsized type on the stack
    let a: &[u8] = &[1,2,3];
    println!("First element of the slice: {}", a[0]);

    // Tuple structs
    struct MyTuple(bool, u32, f64);
    let my_tuple = MyTuple(true, 42, 6.12);
    println!("{}", my_tuple.0);

    // Structs
    struct MyStruct {
        a: bool,
        b: u32,
        c: f64,
    }

    let my_struct = MyStruct {
        a: true,
        b: 42,
        c: 6.12,
    };

    println!("{}", my_struct.a);

    // Enums
    enum Color {
        Red,
        Green,
        Yellow,
        Blue,
    }

    let color = Color::Red;
    if let Color::Red = color {
        println!("Color is Red");
    }

     // 'use' can be used so manual scoping is not needed
     use Color::{Red, Green};
     // Equivalent to 'Color::Red'
     let color = Red;

    // Type aliases
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    let x = Operations::Add;

    // Tagged Unions
    enum Shape {
        Square {
            side: f64,
        },
        Rectangle {
            width: f64,
            height: f64,
        },
        Circle {
            radius: f64,
        },
    }

    let s = Shape::Square {
        side: 4.0,
    };
    match s {
        Shape::Square { side } => {
            println!("A {}x{} square!", side, side);
        },
        Shape::Rectangle { width, height } => {
            println!("A {}x{} rectangle!", width, height);
        },
        Shape::Circle { radius } => {
            println!("A circle of radius {} and diameter {}!", radius, radius * 2.0);
        }
    }

    // Immutable references

    struct Config {
        port : u32,
    };

    let config: Config = Config {
        port: 8080,
    };

    let config_references : &Config = &config;
    println!("Port: {}", config_references.port);

    // We can have multiple immutable references

    let val = 10;
    let r1 = &val;
    let r2 = &val;
    println!("{r1} should be equal to {r2}");

    // Mutable Refernces 

    let mut config: Config = Config {
        port: 8080,
    };
    let config_references : &mut Config = &mut config;
    config_references.port = 4000;
    println!("Port: {}", config.port);

    // We can have only one mutable refernces to a value simultaneously
    let mut val = 10;
    let r1 = &mut val;
    //let r2 = &mut val;
    *r1 = 5;
    //*r2 = 6; // Error: cannot borrow `val` as mutable more than once at a time

    // dereferencing
    let val: i32 = 10;
    let r1: &i32 = &val;

    let val2: i32 = *r1;
    println!("Value: {}", val2);

    // The above code only works with a copyable type. A copyable type is one where we can create a new value by simplying copying the all the bits.
    // For example, string is not a copyable type, because it is contains a pointer to a memory location. So, we cannot copy the string by copying the bits.

    // Lifetimes describe in what part of the code a reference can be safely used. Basically, lifetimes describe the scope of a reference.

    // Implicit conversion is  not allowed in Rust, but implicit conversion is allowed in rust
    let n = 67.66_f32;
    let m = n as u8;
    println!("{m}");
    // There are limitations to implicit conversion, for example we cannot convert a float to a char.

    // Control Flow
    let should_print = true;
    if should_print {
        println!("Should print");
    }

    let value = 10;
    if value == 0 {
        println!("Value is zero");
    } else if value > -10 && value < 10 {
        println!("Single Digit!");
    } else {
        println!("Not a single digit!");
    }

    // Looping -  There are three types of loops in Rust - loop, while, for

    let mut i = 10;
    loop {
        if i == 0 {
            break;
        }
        println!("{i}..");
    }

    while i != 0 {
        println!("{i}..");
        i -= 1;
    }

    for i in (1..=10).rev() {
        println!("{i}..");
    } 
    // 1..=10 ia a range expression that creates a range from 1 to 10. a..b creates a range from a to b-1. .rev() produces a reverse iterator.

    for i in (1..=10).rev() {
        if i % 2 == 0 {
            continue;
        }
        println!("{i}...");
    }

    // Conversions - From and Into provide a way to convert between types. From is used for conversion from a type to another type. Into is used for conversion from a type to another type.
    // TryFrom and TryInto are used for fallible conversions. Fallible conversions are conversions that might fail.

    
}
