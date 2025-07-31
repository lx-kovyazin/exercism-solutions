#[derive(Debug)]
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Self::gen_name(&mut String::new()),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::gen_name(&mut self.name);
    }

    fn gen_name(old_name: &mut String) -> String {
        use once_cell::sync::Lazy;
        use std::{collections::HashSet, sync::Mutex, iter::repeat};
        use rand::{thread_rng, Rng};

        // The cache of names of all robots.
        static mut NAMES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

        // Remove old_name if it was generated earlier.
        if unsafe { !NAMES.lock().unwrap().remove(old_name) } {
            old_name.clear();
        }
        // Generate a new name.
        let name: String = [('A'..='Z', 2), ('0'..='9', 3)]
            .into_iter()
            .flat_map(|(range, num)| repeat(range).take(num).map(|range| thread_rng().gen_range(range)))
            .collect();
        // Cache the generated name if it's really new..
        if unsafe { NAMES.lock().unwrap().insert(name.clone()) } {
            name
        } else { // ..or generate again.
            Self::gen_name(old_name)
        }
    }
}
