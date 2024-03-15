use crate::garden::vegetables::Spinach;
mod garden;

fn main() {
    let plant = Spinach {};
    println!("Im growing {plant:?}");
}
