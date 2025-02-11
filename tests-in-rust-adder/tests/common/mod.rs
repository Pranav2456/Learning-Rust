pub fn setup() {
    println!("Setting up...");
    // This will not be run when the tests are executed.
    // Rust doesn't treat the common module as an integration test file as these 
    // are used to share code between integration tests.
}