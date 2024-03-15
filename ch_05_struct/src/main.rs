use std::fmt::Debug;

fn using_struct_update_syntax() {
    #[derive(Debug)]
    struct User {
        name: String,
        email: String,
        active: u32,
        alive: bool,
    }

    let rust = User {
        name: String::from("Rust"),
        email: String::from("rang-lang.org"),
        active: 1,
        alive: true,
    };
    let python = User {
        name: String::from("Python"),
        email: String::from("python.org"),
        ..rust
    };
    println!("{:#?}\n{:#?}", rust, python);
}

fn main() {
    using_struct_update_syntax();
}
