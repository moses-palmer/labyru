use serde::Deserialize;

use maze::initialize;

/// A random seed.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(transparent)]
pub struct Seed {
    /// The LFSR initialised with the seed.
    lfsr: initialize::LFSR,
}

impl Seed {
    pub fn random() -> Self {
        Self {
            lfsr: initialize::LFSR::new(rand::random()),
        }
    }
}

impl initialize::Randomizer for Seed {
    fn range(&mut self, a: usize, b: usize) -> usize {
        self.lfsr.range(a, b)
    }

    fn random(&mut self) -> f64 {
        self.lfsr.random()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        assert_eq!(
            Seed {
                lfsr: initialize::LFSR::new(1234)
            },
            serde_urlencoded::from_str::<Vec<(String, Seed)>>("seed=1234")
                .unwrap()[0]
                .1,
        );
    }
}
