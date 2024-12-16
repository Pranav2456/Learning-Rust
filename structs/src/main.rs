#[derive(Debug)] // This is a derive annotation, which will allow us to print the struct using the debug formatter.
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods are similar to functions, but they are defined within the context of a struct. They are defined using the impl keyword, followed by the struct name, and then the method name.
// Methods. Below we will an area method for the Rectangle struct. This would be better than a function that takes a Rectangle as an argument.

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Self{
        return Self {
            width: size,
            height: size,
        };
    } // To call this method, we would use Rectangle::square(3), instead of rect1.square(3), because this is an associated function, not a method.
}

// All functions defined within impl blocks are associated functions, because they are associated with the struct. 

// In C and C++, -> Operator is used to access the members of a structure using a pointer. In Rust, this is automatically done when using methods on a struct. 

fn main() {
    // A struct, or strcuture, is a custom data type that lets you package together and name multiple related values that make a meaningful group.

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    };

    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    println!("User1: {}", user1.email);

    let mut user2 = User {
        active: false,
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        sign_in_count: 2,
    }; // Mutable struct

    user2.email = String::from("user212@example.com"); // Mutable field
    println!("User2: {}", user2.email);

    fn build_user(email: String, username: String) -> User {
        return User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        };
    }; // Struct field init shorthand

    let user3 = build_user(String::from("user3@example.com"), String::from("user3"));
    println!("User3: {}", user3.email);

    let user4 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    }; // Struct using another struct

    println!("User4: {}", user4.email);

    // let user5 = User {
    //     email: String::from("another1@example.com"),
    //     ..user1
    // }; // .. syntax, this will fill the remaining fields with the values from user1
    // There is a limitation tho in this condition, you wont be able tu use the user1 fields as it uses String type, you can only use the fields that are not moved.

    // Tuple Structs
    
    struct Color(i32, i32, i32); // Used when regular structs are too verbose
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit like structs
    struct AlwaysEqual; // Used when you dont need any data to be associated with the struct,
    // This is useful when implementing traits on the type, which we will see later.

    let subject = AlwaysEqual;

    // Ownership of Struct Data
    // Structs can hold references, but they need to be annotated with lifetimes, which we will see later.
    // Structs can also hold owned types, like String, which will be valid as long as the struct is valid.

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}"); // ;? tells the compiler to use the debug formatter to print the struct.It uses an output format known as Debug, which is designed for use in debugging scenarios.
    println!("The area of the rectangle is: {}", area1(&rect1));
    println!("The area of the rectangle is: {}", rect1.area());
    println!("rect1 is {rect1:#?}"); // ;#? tells the compiler to use the pretty debug formatter to print the struct.

    // Usage of !dbg 
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // here dbg! will print the value of 30 * scale and return the value of 30 * scale
        height: 50,
    };

    dbg!(&rect2); // We do not want the !dbg to take ownership of the struct, so we pass a reference to it. This will print the struct using the debug formatter.
}

fn area1(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}