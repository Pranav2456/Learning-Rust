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
}
