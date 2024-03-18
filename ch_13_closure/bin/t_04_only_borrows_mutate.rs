fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining the closure {list:?}");
    let mut borrow_mutate = || list.push(4);
    // println!("Before calling the closure {list:?}");
    borrow_mutate();
    println!("After calling the closure {list:?}");
}
