use ch_10a_traits::{Summary, Tweet};

fn main(){
    let tweet = Tweet{
        username: String::from("guardian"),
        content: String::from("something is there!"),
        reply:false,
        retweet:false
    };

    println!("1 new tweet: {}",tweet.summarize()) ;
}