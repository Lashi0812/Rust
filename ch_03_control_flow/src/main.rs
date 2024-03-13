fn return_from_loop(mut x: u32) -> u32 {
    loop {
        x += 1;
        if x == 10 {
            break x; // break and return the value
        }
    }
}

fn if_in_let_statement() {
    println!("Using the if the let statement");
    let condition: bool = true;
    let number = if condition { 5 } else { 6 };
    println!("{number}");
}

fn naming_the_loop() {
    // we can name the loop
    'outer: loop {
        'inner: for i in 1..10 {
            if i == 2 {
                println!("breaking the inner loop");
                break 'inner;
            }
        }
        println!("breaking the outer loop");
        break 'outer;
    }
}

fn looping_through_collections() {
    let arr: [u32; 5] = [10; 5];
    for (idx, val) in arr.iter().enumerate() {
        println!("value at {idx} : {val}");
    }
}

fn main() {
    let value = return_from_loop(5);
    println!("value return form the function : {value}");
    if_in_let_statement();
    naming_the_loop();
    looping_through_collections();
}
