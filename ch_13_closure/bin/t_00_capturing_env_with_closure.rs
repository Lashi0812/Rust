use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut shirt_count: HashMap<ShirtColor, i32> = HashMap::new();
        for shirt in &self.shirts {
            shirt_count
                .entry(shirt.clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        shirt_count
            .iter()
            .max_by_key(|entry| entry.1)
            .unwrap()
            .0
            .clone()
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
