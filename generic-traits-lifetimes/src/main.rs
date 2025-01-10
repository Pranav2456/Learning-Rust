// Generics, Traits, Lifetimes - Rust
// Generics - These are basically abstract stand-ins for concrete types or other properties. 
// They allow you to write code that works with multiple types without having to write separate code for each type.
// Traits - These are a way to define behaviour in a generic way. They are similiar to interfaces in other languages.
// They allow you to define a set of methods that a type must implement in order to be considered a member of that trait.
// Lifetimes - These are a way to ensure that references in Rust are always valid. 
// They are a way to ensure that references are always valid for the lifetime of the reference.

// IMPORTANT
// Traits and trait bounds let us write code that uses generic type parameters to reduce duplication 
// but also specify to the compiler that we want the generic type to have particular behavior.

use std::fmt::Display;
use std::fmt::Debug;

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// We can also define structs to use a generic type parameter in one or more fields using the <> syntax.

struct Point<T> {
    x: T,
    y: T,
}

// We can also define methods on structs that use generics.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// We declared T just after impl so we can use T to specify that we are implementing methods on type Point<T>.

// We can also specify constraints on generic types while defining methods on the type.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
        // powi is a method that raises a number to an integer power.
        // sqrt is a method that returns the square root of a number.
        // If we translate the above code to a equation, it would be sqrt(x^2 + y^2).
        // these operations are only valid for floating point numbers.
    }
}

// A good example of generic parameters in structs and methods.
struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

// We can also define structs with generics that take multiple type parameters.
struct Point2<T, U> {
    x: T,
    y: U,
}

// Using generics, to eleiminate duplication of code.
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}", largest);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}", largest);

    // Above is a classic example of code duplication.
    // To eliminate this duplication, lets create an abstraction using a function.

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    // The above code is way more efficient and less error-prone than the previous code.

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    println!("x = {}", integer.x());
    let float = Point { x: 1.0, y: 4.0 };

    let integer_and_float = Point2 { x: 5, y: 4.0 };
    let float_and_integer = Point2 { x: 1.0, y: 4 };

    // We can also define enums with generics.
    /*
    enum Option<T> {
        Some(T),
        None,
    }
    This syntax makes much more sense now that we know about generics.
    Option<T> is an enum that can have one of two values: Some, which holds a value of type T, and None, which holds no value.

    Enums can also have multiple generic types as well.
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
     */

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Traits
    // Traits are a way to define behaviour in a generic way. They are similiar to interfaces in other languages.
    // Example of a trait.

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // We can also do default implementations for traits.
    pub trait Summary2 {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    // Now, any type that implements the Summary2 trait will have a default implementation of the summarize method.
    // If we want to override the default implementation, we can do so by providing an implementation of the summarize method for that type.
    // Default implementations can call other methods in the same trait, even if those other methods do not have a default implementation.
    // In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it.

    // Here, we declare a trait using the trait keyword. We the define a method and instead providing an implementation, we use a semicolon.
    // Now here this trait means that the compiler will enforce that any type that implements this trait will have a method called summarize, defined within
    // this trait.
    // A trait can have multiple methods in it.

    // Implementing a trait on a type.
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    // impl Summary for Tweet {
    //     fn summarize(&self) -> String {
    //         format!("{}: {}", self.username, self.content)
    //     }
    // }

    impl Summary2 for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    // It is not possible to call the default implementation from an overriding implementation of the same method.

    // Implementing a trait on a type is similiar to implementing regular methods. The difference is that after impl we put the trait name we want to 
    // implement, then use the for keyword, and then specify the name of the type we want to implement the trait for.

    // Traits as parameters.
    // We can also use traits as parameters in functions, which allows us to write functions that accept any type that implements a particular trait.
    // This is useful when we want to write a function that can accept multiple types that implement the same trait.
    pub fn notify1(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    // Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. This parameter accepts any type that implements the
    // the specified trait.  In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize.
    // Code that calls the notify with any other function, like String or i32, will not compile because those types do not implement the Summary trait.

    // The impl Trait syntax is convenient and makes for more concise code in simple cases. However, this syntax is actually syntactic sugar for a longer form:
    // known as a trait bound. The longer form is useful when working with more complex types.
    pub fn notify2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // Specifying multiple trait bounds with the + syntax.
    pub fn notify3(item: &(impl Summary + Display)) {
        // --snip--
    }
    // The body of notify can call summarize as well as any methods on Display because we specified that item must implement both traits.

    // Cleaner trait bounds using where clauses.
    // Using too many trait bounds has its downsides. Each generic has its own trait bounds, which can make function signatures longer and harder to read.
    // for example:
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        // --snip--
        return 9;
    }
    // This is where the where clause comes in. We can use a where clause to clean up the function signature.

    fn cleaner_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        // --snip--
        return 9;
    }
    // This code is much easier to read. The where clause makes it clear that both T and U must implement Display and Clone, and U must also implement Debug.

    // Returning types that implement traits.
    // We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait.
    fn returns_summarizable() -> impl Summary2 {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }

    // Using trait bounds to conditionally implement methods.
    // We can also use trait bounds to conditionally implement methods on a generic type.
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl <T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    // The Pair<T> struct has a method cmp_display that will only be available to Pair<T> instances where T implements the Display and PartialOrd traits.

    // Blanket implementations with trait bounds.
    // We can also implement a trait on any type that satisfies the trait bounds. This is called a blanket implementation.
    // For example, the standard library implements the ToString trait on any type that implements the Display trait.
    /*
    impl<T: Display> ToString for T {
        // --snip--
    }
     */

    // Validating references with lifetimes.
    // Lifetimes are another kind of generic that we have already been using. Rather than ensuring a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.

    // The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data its inteded to reference.

    /*
     let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {r}");

    An attempt to use a reference whose value has gone out of scope.
     */

    // The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // } // Wrong implementation


    // The above code will not compile because the return type of the function is a reference to one of the parameters, which will not work because the parameters
    // have different lifetimes. The function signature does not specify that the return type shares the same lifetime with the parameters.
    // To fix this, we can use lifetime annotations.
    // Lifetime annotations do not change how long any of the references live. They simply describe the relationships of the lifetimes of multiple references.

     // &i32        // a reference
     // &'a i32     // a reference with an explicit lifetime
     // &'a mut i32 // a mutable reference with an explicit lifetime

     fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
     }
     // The longest function specifying that ass the references in the signature must have the same lifetime 'a.
     // In practice, it means that the lifetime of the reference returned by the longest function is the same as
     // the smaller of the lifetimes of the values reffered to by the function arguments.
     // In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y.

     let string1 = String::from("long string is long");

     {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
     }

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");
    // This will not compile because we are attempting to use string2 after it has gone out of scope.

    // Ultimately , lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
    // Once they are connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

    // Lifetime annotations in struct definitions.

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    // Lifetime elision.
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
    
        &s[..]
    }
    // The above code compiles without any lifetime annotations because of a set of rules called lifetime elision.
    // The reason this compiles without lifetime annotations is historical. Lifetime elision was added to Rust before the 1.0 release to reduce the amount of
    // explicit lifetime annotations in functions. The elision rules do not provide full inference. If Rust deterministically applies the rules and is left with
    // lifetime parameters that have not been defined, it will error.
    // The compiler uses three rules to figure out what lifetimes references have when there are no explicit annotations.
    // The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter.
    // A function with two parameters gets two lifetime parameters, and so on.
    // The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
    // The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

    // Lets pretend we are the compiler and apply the lifetime elision rules to the first_word function.
    // The first rule gives the &str parameter its own lifetime parameter, 'a, because the function has one parameter.
    // The second rule states that because there is exactly one input lifetime parameter, 'a, that lifetime is assigned to the output lifetime parameters.
    // The third rule does not apply because there are no references to self.

    // Now for the longest function.
    // The first rule gives each parameter its own lifetime because the function has multiple parameters.
    // The second rule does not apply because there is more than one input lifetime.
    // The third rule does not apply because there are no references to self.
    // After working through all three rules, we still haven't figured out what the lifetime of the return value is. This is why we need to add lifetime annotations to the function signature.

    // Lifetime annotations in method definitions.
    // Lifetime annotations in method definitions are similar to those in function definitions, but there is an extra lifetime parameter for methods.
    // The lifetime of self is assigned to all output lifetime parameters.

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    // The lifetime parameters after impl and its name are required, but the lifetime parameters after the method name are optional because of the third lifetime elision rule.

    impl <'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    // The second rule applies to the announce_and_return_part method because it has one input lifetime parameter, so the output lifetime will be the same as the input lifetime.
    // The lifetime of the return value is the same as the lifetime of self, because of the third lifetime elision rule.

    // The Static Lifetime.
    // The 'static lifetime is a special lifetime that lives for the entire duration of the program. All string literals have the 'static lifetime.
    let s: &'static str = "I have a static lifetime.";
    // The text of this string is stored directly in the program's binary, which is always available. Therefore, the lifetime of all string literals is 'static.

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together.
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where 
    T: Display, 
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

/*
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

Above is an example of how we can use the trait we defined in a different file.
User must bring the trait into scope as well as the type they want to use the trait with.
*/

// Performance of Code using Generics.
// Generic types wont make your program run slower than it would with concrete types.
// rust accomplishes this by performing monomorphization. Monomorophization is the process of turning generic code into specific code by filing in the concrete types 
// the generic code is being called with.



