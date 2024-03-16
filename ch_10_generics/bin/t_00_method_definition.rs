struct Point<T> {
    x: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main() {
    let p = Point { x: 5 };
    println!("p.x = {} ", p.x())
}
