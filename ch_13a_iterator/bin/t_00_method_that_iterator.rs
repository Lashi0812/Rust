fn main() {
    let v1 = vec![1, 2, 3];
    let sum: i32 = v1.iter().sum();
    println!("sum of vec = {v1:?} is {sum}")
}
