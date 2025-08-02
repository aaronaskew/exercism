use std::{collections::HashSet, sync::Mutex};

use rand::Rng;

lazy_static::lazy_static! {
    static ref NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: Self::unique_random_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::unique_random_name();
    }

    fn unique_random_name() -> String {
        const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut name;
        loop {
            name = String::new();
            for _ in 0..2 {
                name.push(
                    ALPHABET
                        .chars()
                        .nth(rand::thread_rng().gen_range(0..26))
                        .unwrap(),
                );
            }
            for _ in 0..3 {
                name.push_str(&rand::thread_rng().gen_range(0..10).to_string());
            }

            let mut names = NAMES.lock().unwrap();

            if !names.contains(&name) {
                names.insert(name.clone());
                break;
            }
        }

        name
    }
}
