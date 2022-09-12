#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Virginia,
}


#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    let coin_value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    };
    coin_value
}

#[test]
fn test_value_in_cents() {
    let penny_value = value_in_cents(Coin::Penny);
    let nickel_value = value_in_cents(Coin::Nickel);
    let dime_value = value_in_cents(Coin::Dime);
    let quarter_value = value_in_cents(Coin::Quarter(UsState::Virginia));
    
    assert_eq!(penny_value, 1);
    assert_eq!(nickel_value, 5);
    assert_eq!(dime_value, 10);
    assert_eq!(quarter_value, 25);
}

fn show_value_in_cents() {

    let penny_value = value_in_cents(Coin::Penny);
    let nickel_value = value_in_cents(Coin::Nickel);
    let dime_value = value_in_cents(Coin::Dime);
    let quarter_value = value_in_cents(Coin::Quarter(UsState::Virginia));
    
    println!("penny_value = {:?}", penny_value);
    println!("nickel_value = {:?}", nickel_value);
    println!("dime_value = {:?}", dime_value);
    println!("quarter_value = {:?}", quarter_value);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    let match_value = match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    match_value
}

#[test]
fn test_plus_one() {
    let x0 = Some(5);
    let x1 = Some(6);
    let x2 = None;

    assert_eq!(plus_one(x0), Some(6));
    assert_eq!(plus_one(x1), Some(7));
    assert_eq!(plus_one(x2), None);
}

fn match_all(value: i32) -> i32 {
    match value {
        5 => 10,
        10 => 20,
        _ => -1,
    }
}

#[test]
fn test_match_all() {
    assert_eq!(match_all(5), 10);
    assert_eq!(match_all(10), 20);
    assert_eq!(match_all(20), -1);
    assert_eq!(match_all(30), -1);
}




fn main() {
    show_value_in_cents();
}
