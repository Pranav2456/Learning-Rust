fn main() {
    // Variable scope
    // s is not valid here, it’s not yet declared
    let s = "hello"; // s is valid from this point forward

    // perform some action with s

    // String Type - Best to illustrate ownership rules

    let s = String::from("hello"); // Method Syntax - Here we are calling a method on a type with the double colon syntax. This is similar to the way you call a function using the double colon syntax with a module that is associated with the function.

    // Such kind of strings can be mutated
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print hello, world!

    // Memory and Allocation
    // In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership features slow down your program while it’s running.
    // In the case of string literals, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
    // The memory must be requested from the operating system at runtime.
    // We need a way of returning this memory to the operating system when we’re done with our String.
    // Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

    // Lets take a complex example
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}"); // This will throw an error because s1 is no longer valid. This is because Rust considers s1 to be invalid after s2 is assigned to s1.

    // Now if the above code was just using integers, the above code would be copying the value of x to y.
    // But here in the case of s1 and s2, heap data is involved. The data on the heap is copied, but the pointer, length, and capacity are copied, not the data itself. This means that s1 and s2 are both pointing to the same memory location.
    // Now in this case both s1 and s2 point to the same memory location. This is a problem because when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs that Rust prevents.
    // To ensure memory safety, after s1 is assigned to s2, Rust considers s1 to be invalid and Rust doesn’t need to free anything when s1 goes out of scope. Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
    // The above method is known as a move.

    // When you assign a new value to an existing variavble, Rust will call drop and free the original value's memory immediately.
    // Example

    let mut s = String::from("hello");
    s = String::from("world");

    println!("{s}");

    // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
    let s1 = String::from("hello");
    let s2 = s1.clone(); // This is expensive in terms of runtime performance

    println!("s1 = {s1}, s2 = {s2}");

    // Stack-Only Data: Copy
    // Rust has a special annotation called the Copy trait that we can place on types like integers that are stored on the stack. If a type has the Copy trait, an older variable is still usable after assignment. Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.

    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    // References and Borrowing
    let s1 = String::from("hello");

    // & here references the value of s1
    let len = calculate_length(&s1); // Reference is like a pointer in that its an address we can follow to access the data stored at that address. But it differs from a pointer in that it is a safe pointer. 
    // Rust ensures that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

    println!("The length of '{s1}' is {len}"); 

    // Mutable References
    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");

    // Mutable refernces have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope. 

    /*
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}"); // This will throw an error
    */

    /*
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3); // This will throw an error

    /*
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
     variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    */

    

    */

      // String slices
  // A string slice is reference to a part of a String
  let s = String::from("hello world");
  println!("{s}");
  let slice = &s[..2]; // If you are starting at the first byte, you can drop the 0 in the range
  println!("{slice}");
  let slice = &s[3..]; // If you are ending at the last byte, you can drop the last number in the range
  println!("{slice}");
  let hello = &s[0..5];// This is a reference to the first five bytes of the string
  let world = &s[6..11]; // This is a reference to the last five bytes of the string
  println!("{hello}, {world}");
    // Multiple immutable references are allowed, but if you have an immutable reference, you cannot have a mutable reference.
    // The benefit of having this restriction is that Rust can prevent data races at compile time.
} // s is no longer valid. // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) -> usize {
  s.len() // Action of creating a refernce is known as borrowing
  // References are immutable by default, we are not allowed to modify the value of the reference
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
// Rust also has a feature called references, which allow you to refer to some value without taking ownership of it.
// This is useful when you want to allow a function to modify a value, but don’t want to take ownership of the value.

// Dangling References

fn dangle() -> String { // dangle returns a reference to a String

  let s = String::from("hello"); // s is a new String

  // &s // we return a reference to the String, s
  s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

// Rules for References
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.