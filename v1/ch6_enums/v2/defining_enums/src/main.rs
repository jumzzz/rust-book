#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}


#[derive(Debug)]
#[derive(PartialEq)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    fn status(&self) {
        println!("status = {:?}", self);
    }
}

fn check_status() {
    
    let quit_msg = Message::Quit;
    let move_msg = Message::Move {x: 30, y: 30};
    let write_msg = Message::Write(String::from("This is a message."));
    let color_msg = Message::ChangeColor(0,255,0);

    quit_msg.status();
    move_msg.status();
    write_msg.status();
    color_msg.status();
}



fn concise_enums() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("home = {:?}", home);
    println!("loopback = {:?}", loopback);
} 


fn some_sample() {

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("some_number = {:?}", some_number);
    println!("some_char = {:?}", some_char);
    println!("absent_number = {:?}", absent_number);
}


fn main() {
    concise_enums();

    println!("Check Status Function...");
    check_status();

    println!("function_call = some_sample()");
    some_sample();
}
