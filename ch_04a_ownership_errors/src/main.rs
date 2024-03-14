//  //*Data must outlive all its reference
// fn error_return_reference() -> &String{ // !s is tied to local scope , when function exist that that hold by s dropped , but the reference still lives even after the data dropped
//     let s = String::from("hello");
//     &s
// }

use std::rc::Rc;

fn fix_return_ownership() -> String {
    println!("\t1. Return the Ownership of data");
    let s = String::from("hello");
    s
}

fn fix_return_string_literal() -> &'static str {
    println!("\t2. Return the static string literal");
    "hello"
}

fn fix_use_reference_counting() -> Rc<String> {
    println!("\t3. Return the reference counting objects");
    let s = Rc::new(String::from("hello"));
    Rc::clone(&s)
}

fn fix_caller_pass_the_mutable_slot(s: &mut String) {
    println!("\t4. Caller pass the mutable slot");
    s.replace_range(.., "Hello");
}

fn error_due_return_reference_to_stack() {
    println!("\n1. ERROR DUE TO RETURNING A REFERENCE TO THE STACK");
    // //!we are violating the data must outlive all its reference
    // //? this caused the local variable of the function tied to function scope.
    // //? when we return the reference of that the local variable , reference outlive the data
    // error_return_reference();
    fix_return_ownership();
    fix_return_string_literal();
    fix_use_reference_counting();
    let mut s = String::new();
    fix_caller_pass_the_mutable_slot(&mut s);
}

// fn stringify_name(name: &Vec<String>) -> String { //* name has permission R,O
//     name.push(String::from("something"));  //* push need permission R,W
//     name.join(" ")
// }

fn error_trying_to_mutate_read_only_data() {
    println!("\n\n2. ERROR DUE TO TRYING TO MUTATE THE READ ONLY DATA");
    // //? By default we read and own permission to variable
    // //? This error often happen when we passed the argument as read only and trying to mutate the data in the function
    // let vec = vec![String::from("Rust"),String::from("book")];
    // stringify_name(&vec);
}

fn pass_mutable_reference(name: &mut Vec<String>) -> String {
    name.push(String::from("world"));
    name.join(" ")
}

fn fix_change_to_mutable_reference() {
    println!("\t1. Pass the mutable Reference!");
    let mut vec = vec![String::from("hello")];
    pass_mutable_reference(&mut vec);
}

fn take_ownership(mut name: Vec<String>) -> String {
    name.push(String::from("world"));
    name.join(" ")
}

fn fix_take_ownership() {
    println!("\t2. Take ownership");
    let vec = vec![String::from("hello")];
    take_ownership(vec);
}

fn clone_then_join(name: &Vec<String>) -> String {
    let mut cloned = name.clone();
    cloned.push(String::from("world"));
    cloned.join(" ")
}

fn fix_clone_then_join() {
    println!("\t\ta. Clone then join");
    let vec = vec![String::from("hello")];
    clone_then_join(&vec);
}

fn join(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" world");
    full
}

fn fix_join() {
    println!("\t\tb. Join the data");
    let vec = vec![String::from("hello")];
    join(&vec);
}

fn fix_clone_data() {
    println!("\t3. Clone data");
    fix_clone_then_join();
    fix_join();
}

fn error_due_not_enough_permission() {
    error_trying_to_mutate_read_only_data();
    fix_change_to_mutable_reference();
    fix_take_ownership();
    fix_clone_data();
}

fn main() {
    error_due_return_reference_to_stack();
    error_due_not_enough_permission();
}
