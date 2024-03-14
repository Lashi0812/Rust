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

fn fix_caller_pass_the_mutable_slot(s :&mut String){
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

fn main() {
    error_due_return_reference_to_stack();
}
