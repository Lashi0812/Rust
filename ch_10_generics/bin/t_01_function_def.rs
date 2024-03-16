struct Mixer<T, U> {
    first: T,
    second: U,
}

impl<T1, U1> Mixer<T1, U1> {
    fn mix<T2, U2>(self, other: Mixer<T2, U2>) -> Mixer<T1, U2> {
        Mixer {
            first: self.first,
            second: other.second,
        }
    }
}

fn main() {
    let m1 = Mixer {
        first: 1,
        second: 2,
    };
    let m2 = Mixer {
        first: "first",
        second: "second",
    };

    let m3 = m1.mix(m2);
    println!("m3.first = {0} ,m3.second = {1}", m3.first, m3.second)
}
