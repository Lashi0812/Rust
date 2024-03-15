#![allow(dead_code)]
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

fn struct_example() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(self: &Self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    println!("{:#?} has area : {}", rect1, rect1.area());
}

fn struct_can_have_multiple_impl_block() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(self: &Self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Self) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("can rect1 hold rect2? : {} ", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? : {} ", rect1.can_hold(&rect3));
}

fn associative_function() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Self {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let square = Rectangle::square(30);
    println!("{:#?} has area : {}", square, square.area());
}


fn main() {
    using_struct_update_syntax();
    struct_example();
    struct_can_have_multiple_impl_block();
    associative_function();
}
