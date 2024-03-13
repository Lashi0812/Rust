fn array_live_in_stack() {
    println!("\n1. ARRAY LIVE IN STACK !");
    let a = [0; 100]; // data in the stack
    let b = a; // data is copied since array implement the copy trait
    println!("value of a[0] : {} , b[0] : {}", a[0], b[0]);
}

fn box_live_in_heap() {
    println!("\n2. BOX LIVE IN HEAP !");
    let a = Box::new([0; 50]); // allocate data on heap and pointer on stack
    let b = a; // moved pointer from a to b since box does not have copy implementation
               // !Here after we cannot use variable a since the value is used
    println!("value of b[0] : {}", b[0]);
}

fn collection_use_box() {
    println!("\n3. COLLECTION USE BOX");
    let a = String::from("hello"); // allocate data in heap and pointer in heap
    let b = a; // pointer is moved to b
               // !Here after we cannot use variable a since the value is used
    println!("value in b {b}");
}

fn clone_avoids_move() {
    println!("\n4. CLONE AVOIDS MOVE");
    let a = String::from("hello"); // allocate data in heap and pointer in heap
    let mut b = a.clone(); // new data allocate in heap and pointer returned to b and //* ie. clone perform deep copy
                           // !a and b point different data in heap
    b.push_str(" World");
    println!("a : {a} \nb : {b}");
}

fn greet1(s1: String, s2: String) {
    println!("string s1 : {s1}\nstring s2 :{s2}");
}

fn read_string_twice1() {
    println!("\n5. READ STRING TWICE wont compile");
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    greet1(s1, s2); // s1 and s2 moved
                    // let s = format!("{s1} {s2}"); // !don't compile since value is moved
}

fn greet2(s1: String, s2: String) -> (String, String) {
    println!("string s1 : {s1}\nstring s2 :{s2}");
    (s1, s2)
}

fn read_string_twice2() {
    println!("\n6. READ STRING TWICE2 With return");
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let (s1, s2) = greet2(s1, s2); // s1 and s2 moved and returned
    let s = format!("{s1} {s2}");
    println!("combined string : {s}")
}

fn greet3(s1: &String, s2: &String) {
    println!("string s1 : {s1}\nstring s2 :{s2}");
}

fn read_string_twice3() {
    println!("\n7. READ STRING TWICE USING REFERENCE");
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    greet3(&s1, &s2); // *here we pass the reference of the s1 and s2 and inside the function to access the data in heap it need to jump 2 to get the data
    let s = format!("{s1} {s2}");
    println!("combined string : {s}")
}

fn data_safety_principle() {
    println!("\n8. DATA SAFETY PRINCIPLE");
    let mut v = vec![1, 2, 3, 4];
    let num = &v[3];
    v.push(5);
    // println!("fourth element : {}", *num); //! data can alias , data can mutate , but not both
}

fn main() {
    array_live_in_stack();
    box_live_in_heap();
    collection_use_box();
    clone_avoids_move();
    read_string_twice1();
    read_string_twice2();
    read_string_twice3();
    data_safety_principle(); // !data can alias , data can mutate , but not both
}
