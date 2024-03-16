fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let str1 = String::from("hello");
    let str2 = "hi";
    let result = longest(&str1, &str2);
    println!("the longest string is {result}");
}
