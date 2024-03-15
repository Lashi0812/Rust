#![allow(dead_code)]

use rand::Rng;

fn match_as_control_flow() {
    enum Coin {
        One,
        Two,
        Five,
        Ten,
    }

    let one = Coin::One;

    match one {
        Coin::One => println!("you have got one"),
        Coin::Two => println!("you have got two"),
        Coin::Five => println!("you have got five"),
        Coin::Ten => println!("you have got ten"),
    }
}

fn match_pattern_bind_the_values() {
    fn print_ip_addr(addr: &IpAddr) {
        match addr {
            IpAddr::V4(a, b, c, d) => println!("Ipv4 : {a}.{b}.{c}.{d}"),
            IpAddr::V6(full) => println!("IPv6 : {full}"),
        }
    }
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    print_ip_addr(&home);
    print_ip_addr(&loopback);
}

fn using_option_to_handle_none() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} + 1 = {:?}\nNone + 1 = {:?}", five, six, none,);
}

fn catch_all_pattern() {
    loop {
        let roll = rand::thread_rng().gen_range(1..7);
        match roll {
            6 => {
                println!("Yeah we got 6 so start the game");
                break;
            }
            other => {
                println!("you have got {other} so reroll!")
            }
        }
    }
}

fn enum_match_example() {
    enum Location {
        Point(i32),
        Range(i32, i32),
    }

    let l = Location::Range(0, 5);
    let n = match l {
        Location::Point(_) => -1,
        Location::Range(_, n) => n,
        Location::Range(0, _) => 0,
        _ => -2,
    };
    println!("{n}")
}


fn main() {
    // match_as_control_flow();
    // match_pattern_bind_the_values();
    // using_option_to_handle_none();
    // catch_all_pattern();
    enum_match_example();
}
