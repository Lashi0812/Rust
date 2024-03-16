use ch_10a_traits::{Summary, Tweet};

fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
fn main() {
    let tweet = Tweet {
        username: String::from("let's break"),
        content: String::from("flash news"),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
}
