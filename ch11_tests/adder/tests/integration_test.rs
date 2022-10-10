use adder::util_funcs::*;
use adder::sample_struct::Guess; 


#[test]
fn it_adds_two_v2() {
    println!("Executing tests/integration_test.rs");
    assert_eq!(6, add_two(4));
}

#[test]
fn greeting_contains_name_v2() {
    let result = greeting("Gatching");
    println!("result = {}", result);

    assert!(result.contains("Gatching"));
}

