fn string_slice() {
    let a = String::from("Hello");
    println!("full = {} ; 1..3 = {}", &a[..], &a[1..3]);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn string_literal_is_string_slice() {
    let a = "hello";
    print_type_of(&a);
}

fn main() {
    string_slice();
    string_literal_is_string_slice();
}
