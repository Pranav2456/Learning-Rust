fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5; //  First variable is shadowed by the second variable, which means that the second variable is what the compiler will see when  you use the name of the variable

    let x = x + 1; // Now here the second x is shadowed by the third x

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");

    }
    println!("The value of x is: {x}");

    // Difference between mut and shadowing
    // With mut, the variable is mutable, and you can change its value whenever you want. With shadowing, the variable is immutable after the first value is set.
    // The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.

    // There are two types of data types in Rust: scalar and compound
    // Scalar types represent a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
    // Integer types : i8, u8, i16, u16, i32, u32, i64, u64, i28, u128 isize, usize
    // Floating Types: f32, f64 - f32 has single precision, f64 has double precision
    // Boolean Type: bool
    // Character Type: char

    // Examples of scalar types
    let x: u32 = 5; // Integer type
    let y: f64 = 3.0; // Floating type
    let z: bool = false; // Boolean type
    let a: char = 'a'; // Character type // four bytes in size. Unicode scalar value, which means it can represent a lot more than just ASCII.

    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
    // Tuple type:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Tuple is a general way of grouping together a number of values with a variety of types into one compound type. They have a fixed length.
    let (x, y, z) = tup; // Destructuring a tuple
    println!("The value of y is: {y}");

    // Accessingh tuple elements by period 
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {five_hundred}");

    // Array Type:
    // Collection of multiple values of the same type. Unlike a tuple, every element of an array must have the same type. Arrays in Rust have a fixed length.
    let a = [1, 2, 3, 4, 5];
    let b = [3; 5]; // This creates an array that contains five elements, all set to the value 3. This is the same as writing let a = [3, 3, 3, 3, 3];

    // Accessing array elements
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {first}");

    // Functions
    another_function(5);

    // Statements and Expressions
    // Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.
    // Expressions evaluate to something and return a value, while statements do not return a value. Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.

    let x = five();
    println!("The value of x is: {x}");

    control_flow();
}

fn another_function(x : i32) {
    println!("Another function. {x}");
}

fn five() -> i32 {
    5
} // functions with a return value have a return type specified after an arrow (->). The return value is the last expression in the function body. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

// Control Flow
fn control_flow() {
    let number = 3;

    let number = 600;

    if number % 4 == 0  {
        println!("number is divisible by 4");
    } else if  number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }


    // Rust does not automatically convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition.

    let condition = true;

    //  Using let with if
    let number = if condition { 5 } else { 6 }; // The values that have the potential to be results from each arm of the if must be the same type.

    println!("The value of number is: {number}");

    // loop
    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter *2;
        }
    };

    println!("The result is: {result}");

    // If we have loops within loops, break and continue apply only to the innermost loop.
    // You can specify a label after break or continue to break or continue the outer loop, as we are doing here with 'counting_up.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9  {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // better approach is to use a for loop
    for number in (1..4).rev() {
        println!("number = {number}");
    } // rev() reverses the range
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // better approach is to use a for loop
    for element in a {
        println!("the value is: {element}");
    }


    // Program to convert temperatures between Fahrenheit and Celsius
    let fahrenheit_temps = [32.0, 0.0, 212.0, 100.0, 98.0];
    let celsius_temps = [0.0, -17.77777777777778, 100.0, 37.77777777777778, 36.666666666666664];

    for (i, &fahrenheit) in fahrenheit_temps.iter().enumerate() {
        let celsius = (fahrenheit - 32.0) / 1.8;
        let expected = celsius_temps[i];

        println!("{} Fahrenheit is {} Celsius", fahrenheit, celsius);
        assert_eq!(celsius, expected);
    }
    // Here lets break the for loop and understand the code
    // fahrenheit_temps.iter() returns an iterator that returns each element of the array in turn. The enumerate method adapts an iterator and returns each element along with its index.
    // (i, &fahrenheit) is pattern matching. The enumerate method returns a tuple, and i is the index, and fahrenheit is the element.

    fibonacci(10);
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut a = 0; // First fibonacci number
    let mut b = 1; // Second fibonacci number

    for _ in 2..=n {
        let temp = a + b;
        a = b; // a changes for each iteration to be the value of b, and b changes to be the value of temp. This is because for each new iteration, we need to consider the next numbers in the sequence.
        b = temp;
    } // The underscore, _, is a special variable in Rust used when you don’t need to name a variable. In this example, we don’t need to use the values generated by the range, so we use _ to ignore them. 
    println!("The value of fibonacci is: {b}");
    b
}