fn print_type_of<T>(_: &T, msg: &str) {
    println!("{} = {}", msg, std::any::type_name::<T>())
}

fn sample_str() {

    println!("\nsample_str()");
    let mut s = String::new();
    let msg = "type of s";

    println!("contents of s = {}", &s);
    print_type_of(&s, &msg);

}

fn sample_str_conv() {
    println!("\nsample_str_conv()");

    let data = "initial contents";
    let msg01 = "type of data";

    print_type_of(&data, &msg01);

    let s = data.to_string();
    let msg02 = "type of s";

    println!("contents of s = {}", &s);
    print_type_of(&s, &msg02);

}

fn print_utf8_strs() {
    println!("\nprint_utf8_strs()");
    let hello = String::from("السلام عليكم");
    println!("{}", &hello);

    let hello = String::from("Dobrý den");
    println!("{}", &hello);

    let hello = String::from("Hello");
    println!("{}", &hello);
    
    let hello = String::from("שָׁלוֹם");
    println!("{}", &hello);
    
    let hello = String::from("नमस्ते");
    println!("{}", &hello);
    
    let hello = String::from("こんにちは");
    println!("{}", &hello);
    
    let hello = String::from("안녕하세요");
    println!("{}", &hello);
    
    let hello = String::from("你好");
    println!("{}", &hello);

    let hello = String::from("Olá");
    println!("{}", &hello);
    
    let hello = String::from("Здравствуйте");
    println!("{}", &hello);

    let hello = String::from("Hola");
    println!("{}", &hello);
    
}

fn print_plain_str() {
    
    println!("\nprint_plain_str()");
    println!("السلام عليكم");
    println!("Dobrý den");
    println!("Hello");
    println!("שָׁלוֹם");
    println!("नमस्ते");
    println!("こんにちは");
    println!("안녕하세요");
    println!("你好");
    println!("Olá");
    println!("Здравствуйте");
    println!("Hola");

}

fn str_appends() {
    let mut s = String::from("foo");

    println!("before push_str = {}", &s);
    s.push_str("bar");
    println!("after push_str = {}", &s);
    s.push('!');
    println!("after push = {}", &s);

}

fn str_concat() {
    let s0 = String::from("Hello, ");
    let s1 = String::from("World!");

    let s2 = s0 + &s1;
    println!("\nConcat result: {}", s2);
}

fn str_format() {
    let s0 = String::from("tic");
    let s1 = String::from("tac");
    let s2 = String::from("toe");

    let s = format!("{}-{}-{}", &s0, &s1, &s2);
    
    println!("{}-{}-{}", &s0, &s1, &s2);
    println!("{}", &s);

}

fn str_index() {
    let s0 = String::from("hello");
    let h = &s0[0..1];

    println!("{:?}", h);
}

fn char_iter() {
    let s0 = String::from("hello");

    for c in s0.chars() {
        println!("{}", &c);
    }
}


fn main() {
//    sample_str();
//    sample_str_conv();
//    print_utf8_strs();
//    print_plain_str();
//    str_appends();
//    str_concat();
//    str_format();
//    str_index();
    char_iter();
}
