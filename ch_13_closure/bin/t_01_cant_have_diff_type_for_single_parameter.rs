fn main() {
    let cant_have_multiple_type = |x| x;
    let s = cant_have_multiple_type(String::from("hello"));
    
    
    // !type is locked to string
    let i = cant_have_multiple_type(5);
}
