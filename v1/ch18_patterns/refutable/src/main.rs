fn irrefutable() {
    if let x = 5 {
        println!("x = {} is irrefutable.", x);
    }
}

fn refutable() {
    let some_value: Option<&str> = Some("Refute me");
    if let Some(x) = some_value {
        println!("some_value has value");
    } else {
        println!("some_value has none");
    }
}

fn main() {
    // irrefutable();
    refutable();
}
