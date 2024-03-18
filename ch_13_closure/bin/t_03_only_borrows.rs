fn main() {
    let list = vec![1, 2, 3];
    println!("before defining the closure {list:?}");
    let only_borrows = || println!("from closure: {list:?}");
    println!("before calling the closure {list:?}");
    only_borrows();
    println!("after calling the closure {list:?}")
}
