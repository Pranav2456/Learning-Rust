pub fn add(left: usize, right: usize) -> usize {
    internal_adder(left, right)
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello! {}", name)
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}
 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// The below code is an example of tests in Rust.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        // assert_eq macro is used to test equality, in this case if the result is equal to 4
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2,2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("The Result is not correct"))
        }
    }

    #[test] 
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // assert is basically used to as an assertion, basically if an assert returns true, it passes, if its the opposite it fails, it basically used
        // to define assertions in Rust.
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // Here we need to negate the result as the original result is false.
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Name");
        assert!(result.contains("Name"),
        "Greeting did not contain name, value was {result}")
    }

    // Running Tests in Parallel or Consecutively
    // When you run multiple tests, by default they run in parallel using threads, meaning they finish running faster
    // and you get feedback quicker. Because the tests are running at the same time, you must make sure the tests dont depend on
    // each other or any shared state, including a shared enviornment, suchas the current directory or env's.

    // If we dont want to run the tests in parallel or if you want more fine-grained control over the number of threads used, we can send
    // the --test-threads flag and the number of threads you want to use the tests binary.
    // cargo test -- --test-threads=1

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    #[ignore] // This will ignore the test.
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
    // cargo test -- --show-output - Show the output of the tests.
    // cargo test test_name - Run a single test.
    // cargo test add - Runs all tests that have add in the name.
    // cargo test -- --ignored - Run all ignored tests.

    // Test Organization
    // We can organize pur tests in terms of two main categories: Unit Tests and Integrations Tests.
    // Unit tests are smalla and more focused, testing one module in isolation at a time, and can test private interfaces.
    // Integration tests are entirely external to your library and use your code in the same way any other external code will ue it.

}
