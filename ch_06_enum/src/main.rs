#![allow(dead_code)]
#![allow(unused_variables)]

fn defining_enum() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:#?} {:#?}", four, six);
}

fn using_enum_in_struct() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        addr: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        addr: String::from("::1"),
    };

    println!("{:#?} {:#?}", home, loopback);
}

fn attach_data_to_enum_rather_using_enum_in_struct() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home = {:?} loopback = {:?}", home, loopback);
}

fn each_variant_can_have_diff_types() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home = {:?} , loopback = {:?}", home, loopback);
}

fn using_struct_in_enum() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: u8, y: u8 },
        Write(String),
        ChangeColor(u8, u8, u8),
    }

    let something = Message::Write(String::from("something"));
    let red = Message::ChangeColor(255, 255, 255);

    println!("{:?}\n{:?}", something, red);
}

fn impl_method_for_enum() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: u8, y: u8 },
        Write(String),
        ChangeColor(u8, u8, u8),
    }

    impl Message {
        fn call(&self) {}
    }

    let something = Message::Write(String::from("something"));
    let red = Message::ChangeColor(255, 255, 255);

    println!("{:?}\n{:?}", something, red);
    something.call();
    red.call();
}

fn main() {
    // defining_enum();
    // using_enum_in_struct();
    // attach_data_to_enum_rather_using_enum_in_struct();
    // each_variant_can_have_diff_types();
    // using_struct_in_enum();
    impl_method_for_enum();
}
