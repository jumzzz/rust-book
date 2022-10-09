use traits::NewsArticle;
use traits::Summary;
use traits::Pair;

fn news_article() {
    let news = NewsArticle {
        headline: String::from("Dog bites Biden"),
        location: String::from("Washington DC"),
        author: String::from("Hunter Biden"),
        content: String::from("Haha good dog!"),
    };

    println!("news.summary = {}", news.summarize());

}

fn execute_pair() {
    let pair_v0 = Pair {x: 30, y: 35};
    println!("pair_v0 = {:?}", pair_v0);

    println!("pair_v0 comparison: ");
    pair_v0.cmp_display();

    let pair_v1 = Pair {x: 69, y: 68};
    println!("pair_v1 = {:?}", pair_v1);

    println!("pair_v1 comparison: ");
    pair_v1.cmp_display();

}


fn main() {

    execute_pair();

}
