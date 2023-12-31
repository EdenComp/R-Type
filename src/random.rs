use std::time::{SystemTime, UNIX_EPOCH};

const RANDOM_MULTIPLIER: u128 = 25214903917;
const RANDOM_ADDITION: u128 = 11;

#[derive(Clone)]
pub struct Random {
    last_number: u128,
}

impl Random {
    pub fn new(seed: u128) -> Random {
        Random { last_number: seed }
    }

    pub fn refresh(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error getting time");
        self.last_number = millis.as_micros();
    }

    fn rand(&mut self) {
        self.last_number = self.last_number * RANDOM_MULTIPLIER + RANDOM_ADDITION
    }

    pub fn range(&mut self, min: usize, max: usize) -> usize {
        self.rand();
        (self.last_number % max as u128 + min as u128) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut random = Random::new(0);
        let mut random2 = Random::new(1);

        let num1 = random.range(0, 100);
        let num2 = random.range(0, 100);

        assert_eq!(num1, 11);
        assert_eq!(num2, 98);
        assert_ne!(random.range(0, 100), random2.range(0, 100));
    }

    #[test]
    fn test_same_seed() {
        let mut random = Random::new(0);
        let mut random2 = Random::new(0);

        assert_eq!(random.range(0, 100), random2.range(0, 100));
    }
}
