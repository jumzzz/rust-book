fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn split_sentence(s: &String) -> Vec<&str> {
    let mut split_words: Vec<&str> = Vec::new();

    let mut start_idx: usize = 0;
    let mut end_idx: usize = 0;

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            end_idx = i;
            split_words.push(&s[start_idx..end_idx]);
            start_idx = end_idx;
        }
    }

    split_words.push(&s[start_idx..]);
    split_words

}

#[test]
fn test_split_sentence() {

    let s0 = String::from("hello corgi");
    let s1 = String::from("corgi");
    let s2 = String::from("dog cat goat");

    assert_eq!(split_sentence(&s0), vec![&s0[0..5], &s0[5..]]);
    assert_eq!(split_sentence(&s1), vec![&s1[..]]);
    assert_eq!(split_sentence(&s2), vec![&s2[0..3], &s2[3..7], &s2[7..]]);

}



fn main() {
    let s = String::from("Hello world this is split_sentence");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("Printing hello = {}", hello);
    println!("Printing hello = {}", world);

    let len = s.len();
    
    let slice = &s[3..len]; 
    println!("Slice with explicit ..len = {}", slice);
    
    let slice = &s[3..];
    println!("Slice with implicit range = {}", slice);

    let slice = &s[0..len];
    println!("Explicit whole range = {}", slice);

    let slice = &s[..];
    println!("Implicit whole range = {}", slice);

    let first_word = first_word(&s);
    println!("First word = {}", first_word);

    let split_words = split_sentence(&s);

    println!("Split Sentence = {:?}", split_words);

}
