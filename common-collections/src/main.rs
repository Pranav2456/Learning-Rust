// Rust standard lubrary includes a number of very useful data structures called collections.
// The most often used are Vector, String and HashMap.

use std::collections::HashMap;

fn main() {
    // Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
    // Vectors can only store values of the same type.
    let v: Vec<i32> = Vec::new(); // Creates a new empty vector. Type annotation is needed here because we are initializing an empty vector.

    let v = vec![1,2,3]; // The vec! macro creates a new vector with values.
    println!("{:?}", v);

    // Updating a vector
    let mut v = Vec::new(); // This deosn't need a type annotation because we are adding values to the vector.
    v.push(5);
    v.push(6);
    v.push(7);
    println!("{:?}", v);

    // There are two ways to reference or access elements in a vector: indexing and the get method.
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2]; // Indexing
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2); // get method
    match third {
        Some(third) => println!("The third elemnt is {third}"),
        None => println!("There is no third element"),
    }

    // The get method is much safer than indexing because it returns an Option<&T> rather than a reference.
    // With the get method, if you pass an index that is out of bounds, it will return None without panicking.
    // indexing is used when you want your program to crash if an out-of-bounds index is used.

    /*
    let mut v = vec![1,2,3,4,5];

    let first = &v[0]; // immutable borrow

    v.push(6); // mutable borrow

    println!("The first element is: {}", first);

    This code might look like it should work: Why does a reference to the first vector element care about changes at the end of the vector, but it doesn't,
    because vectors put the values next to each other in memory, and if the vector needs to change size, it will allocate new memory elsewhere, copy the old elements over to the new space, and free the old space.
    This means that the reference to the first element would be pointing to deallocated memory.
    */

     let mut v = vec![1,2,3,4,5];

     v.push(6);

     let first = &v[0];

    println!("The first element is: {}", first);

    // Iterating over the values in a vector

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    // To change the value that the mutable reference refers to, we have to use the * dereference operator
    // to get to the value in i before we can use the += operator.

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    };

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Above is an example how we can use an enum to store multiple types of values in a vector.
    // This is useful when you want to store elements in a vector that are of different types.
    // Using an enum plus a match expression means the compiler can ensure at compile time that you handle all cases, even if there are more cases in the future.

    // Like any other element, vector is freed when it goes out of scope.

    // STRINGS
    // Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str.
    // The String type, which is provided by Rust's standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

    // Use String when you need to own or modify the string data. If you just need to use a string immutably, use the string slice type str.

    // Creating a new empty string
    let mut s = String::new();

    // Creating a new string with data
    let data = "initial contents"; // This is a string literal

    let s = data.to_string(); // to_string method is available on any type that implements the Display trait, which includes string literals.

    // Direct method
    let s = "initial contents".to_string();

    let s = String::from("initial contents"); // This does the same thing as the to_string method.

    // strings are UTF-8 encoded, which means that you can include any properly encoded data in them.
    let hello = String::from("नमस्ते");

    // Updating a string
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str method appends a string slice to a String.

    println!("{}", s);
    // The push_str method takes a string slice because we don't necessarily want to take ownership of the parameter.

    // Concatentaion with the + operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;
    println!("{}", s3);
    // Note: s1 has been moved fere and can no longer be used.
    // The + operator uses the add method, which takes two parameters self, and a &str.
    // So in this case we are passing a reference to string instead of string slice and it still compiles
    // because Rust coerces the &String argument into a &str argument.
    // So as the self is still moved in the case of the original function, in this case, that woud make s1 unusable.
    // So basically this implementation is more efficient than copying.

    // More complex concatenation
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    // Now if we are using + here it's difficult to read and error prone.
    println!("{}", s);

    // redefine s1 as its moved
    let s1 = String::from("tic");

    // Using the format! macro
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // Indexing into strings
    let s1 = String::from("hello");
    // let h = s1[0];
    // This code will not compile because strings are wrappers over a sequence of bytes, and the bytes are not guaranteed to represent valid UTF-8 characters.
    // it will show error "the type str cannot be indexed by {integer}"

    // Internal representation
    // A String is basically a wrapper over a Vec<u8>.

    let hello = String::from("Hola");
    // In this case, the len would be 4, which means the vector string is 4 bytes long.
    // Each of these letters takes one byte when encoded in UTF-8.

    let hello = String::from("Здравствуйте");
    // If we take a look at this string, it looks like the len should be 12, but Rust's answer is 24.
    // This is beacuse each unicode scalar value in that string takes 2 bytes of storage.
    // Therfore, an index into the string's bytes will not always correlate to a valid Unicode scalar value.
    // So Rust doesnt compiler indexing into strings.

    // Bytes, Scalar values, and Grapheme clusters
    // There are three ways to look at strings from Rust's perspective: as bytes, scalar values, and grapheme clusters(closest to what we would call letters).

    // A final reason why Rust doesn't allow us to index into a String to get a character is that indexing operations are expected to always take constant time(O(1)).
    // But that operation is not possible with a string because Rust would have to walk through the string to determine how many valid characters there are.

    // Slicing strings
    // Rather than indexing using [] with a single number, you can use a range to create a string slice containing particular bytes.
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // First four bytes of the string
    println!("{}", s);
    // Caution is needed when using ranges to slice strings.

    // Methods for iterating over strings
    // Best way to operate on pieces of strings is to be explicit about you want characters or bytes.
    // For individual unicode scalar values, use the chars method.
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // This will print "न", "म", "स", "त", "े".

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // This will print the raw bytes of the string.

    // Getting grapheme clusters from strings is complex so they are not provided in the standard library.

    // Other useful methods for strings : contains, clear, is_empty, replace, split, trim, to_lowercase, to_uppercase, etc.

    // HASH MAPS
    // The type HashMap<K,V> stores a mapping of keys of type K to values of type V, using a hashing functions, which determines 
    // how it places these keys and values into memory.
    // Hash maps are useful when you want to lookup data not by using an index, as you can with vectors, but by using a key that can be of any type.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 30);
    scores.insert(String::from("Yellow"), 100);

    println!("{:?}", scores);
    // Hash maps are homogeneous, all keys must have the same type, and all values must have the same type.

    // Accessing values in a hash map.
    // get method
    let team_name = String::from("Blue");
    // 1st and the easiest way
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // The above statement handles the Option by calling copied to get an Option<i32> instead of an Option<&i32>, than
    // unwrapping the Option to get the value inside, and if the Option is None, it returns 0.

    // 2nd way
    let score = match scores.get(&team_name) {
        Some(score) => *score,
        None => 0,
    };
    // The raw way to handle the Option.
    println!("{}", score);

    // Iterating over a hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Ownership in hash maps
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point as they have been moved into the hash map, and
    // hash map now owns them.
    // If we insert references to values into the hash map, the values won't be moved into the hash map and the values that 
    // references point to must be valid for as long as the hash map is valid.

    // Updating a hash map
    // When you want to change the data in hash map, you have to decide how to handle the case when a key already has a value assigned.
    // You could replace the old value with the new value, completely disregarding the old value.
    // You could keep the old value and ignore the new value, only adding the new value if the key doesn't already have a value, 
    // or you could combine the old value and the new value.

    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    // The value for the key "Blue" has been overwritten.

    // Adding a Key and Value Only If a Key Isn't Present
    // HashMaps have a special API for this called entry that takes the key you want to check as a parameter.
    // The return value of the entry method is an enum called Entry that represents a value that might or might not exist.

    scores.entry(String::from("Red")).or_insert(50);
    // The above statement will check if the key "Red" has a value associated with it, and if it doesn't, it will insert the value 50.
    // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.
    println!("{:?}", scores);

    // Another common use case for hash maps is to look up a key's value and then update it based on the old value.
    // We use a hash map with words as keys and increment the value to keep track of how many times we've seen that word.
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // split_whitespace method returns an iterator that produces a series of strings, separated by whitespace.
        let count = map.entry(word).or_insert(0);
        // The above statement returns a mutable reference to the value for this key, and if the key doesn't exist, it inserts the key with a value of 0.
        *count += 1;
        // The above statement dereferences the value in count and increments it.
    }

    println!("{:?}", map);

    // By default, HashMap uses a hashing function called SipHash that can provide resistance to certain denial-of-service(DoS) attacks involving hash tables.
    // This is not the fastest alogrithm available, but the trade-off for better security is worth it.
    // If you profile your code and find that the default hash function is too slow, you can switch to another function by specifying a different hasher.
    // A hasher is a type that implements the BuildHasher trait.
    // crates.io has a number of crates that provide alternative hashers.

    // Given a list of integers, use a vector and return the median and mode of the list.
    let list = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    let median_length = list.len() / 2;
    let median = list[median_length];
    println!("Median: {}", median);

    let mut mode_map = HashMap::new();
    for i in list {
        let count = mode_map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut mode_temp = 0;

    for (key, value) in &mode_map {
        if *value > mode_temp {
            mode_temp = *value;
            mode = *key;
        }
    }
    println!("Mode: {}", mode);
}

fn get_median_mode(mut nums: Vec<i32>) -> (f64, i32) {
    // Sort the vector in-place using unstable sort (faster for integers)
    nums.sort_unstable();
    let mid = nums.len() / 2;
    
    // Calculate median - if even length, average of two middle numbers, else middle number
    let median = if nums.len() % 2 == 0 { 
        (nums[mid - 1] + nums[mid]) as f64 / 2.0 
    } else { 
        nums[mid] as f64 
    };
    
    // Mode calculation step by step:
    let mode = nums.iter()  // Create an iterator over our numbers
        // fold() accumulates values into a HashMap:
        //   - HashMap::new() is our initial empty map
        //   - |mut map, &num| is our closure taking:
        //     - map: our HashMap accumulator
        //     - num: each number from our iterator
        .fold(HashMap::new(), |mut map, &num| {
            // For each number:
            // 1. map.entry(num) - Look up this number in our HashMap
            // 2. .or_insert(0)  - If number isn't in map, insert 0 as its count
            // 3. += 1           - Increment the count by 1
            // Example: for [1,2,2,3], builds HashMap: {1:1, 2:2, 3:1}
            *map.entry(num).or_insert(0) += 1;
            map  // Return map for next iteration
        })
        // Convert HashMap into iterator of (number, count) pairs
        .into_iter()
        // Find the entry with maximum count:
        // |(_, count)| - Pattern match each (number, count) pair
        // *count       - Use count as comparison key
        // Returns Option<(number, count)> of highest count
        .max_by_key(|(_, count)| *count)
        .unwrap()  // Extract (number, count) from Option
        .0;        // Get first element (number) from tuple

    (median, mode)
} // Highly optimized function to get median and mode of a list of integers.
