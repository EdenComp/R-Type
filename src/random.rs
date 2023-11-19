use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

// const RANDOM_MULTIPLIER: u128 = 25214903917;
// const RANDOM_ADDITION: u128 = 11;

pub struct Random {
    last_number: u128,
}

impl Random {
    pub fn new(seed: u128) -> Random {
        Random { last_number: seed }
    }

    // pub fn range(&mut self, min: usize, max: usize) -> usize {
    //     self.last_number = self.last_number * RANDOM_MULTIPLIER + RANDOM_ADDITION;
    //     (self.last_number % max as u128 + min as u128) as usize
    // }

    pub fn random_in_empty_pos(&mut self, vec_empty_pos: &Vec<(i8, i8)>) -> Result<usize, ()> {
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Error get time")
            .as_nanos();

        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let rng = hasher.finish();

        if !vec_empty_pos.is_empty() {
            return Ok (rng as usize % vec_empty_pos.len());
        }
        Err(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_basic() {
//         let mut random = Random::new(0);
//         let mut random2 = Random::new(1);
//         let num1 = random.range(0, 100);
//         let num2 = random.range(0, 100);

//         assert_eq!(num1, 11);
//         assert_eq!(num2, 98);
//         assert_ne!(random.range(0, 100), random2.range(0, 100));
//     }

//     #[test]
//     fn test_same_seed() {
//         let mut random = Random::new(0);
//         let mut random2 = Random::new(0);

//         assert_eq!(random.range(0, 100), random2.range(0, 100));
//     }
// }
