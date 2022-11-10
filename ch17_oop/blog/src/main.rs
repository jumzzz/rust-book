use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    println!("post.add_text");
    println!("{}", post.content());

    post.request_review();

    println!("post.request_review()");
    println!("{}", post.content());
    assert_eq!("", post.content());
    
    post.approve();
    
    println!("post.approve()");
    println!("{}", post.content());
    assert_eq!("I ate a salad for lunch today", post.content());

}
