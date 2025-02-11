use tests_in_rust_adder::add;

#[test]
fn it_adds_two() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

// There are three sections of output: the unit tests, the integration tests and the doc tests.
// If any test in a section fails, the following sections will not run.