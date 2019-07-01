use std::iter;
use std::u64;

use serde::{Deserialize, Serialize};

#[cfg(feature = "osrand")]
use rand;

pub mod randomized_prim;

pub trait Randomizer {
    /// Generates a random value in the range `[low, high)`, where `low` and
    /// `high` are the low and high values of `a` and `b`.
    ///
    /// # Arguments
    /// *  `a` - A number.
    /// *  `b` - A number.
    fn range(&mut self, a: usize, b: usize) -> usize;

    /// Generates a random value in the range `[0, 1)`.
    fn random(&mut self) -> f64;
}

#[cfg(feature = "osrand")]
impl<T> Randomizer for T
where
    T: rand::Rng,
{
    fn range(&mut self, a: usize, b: usize) -> usize {
        if a < b {
            self.gen_range(a, b)
        } else {
            self.gen_range(b, a)
        }
    }

    fn random(&mut self) -> f64 {
        self.next_f64()
    }
}

/// A linear feedback shift register.
#[derive(Deserialize, Serialize)]
pub struct LFSR(u64);

impl LFSR {
    /// Creates a new linear shift register.
    ///
    /// # Arguments
    /// *  `seed` - The seed. This value will not be yielded.
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// Advances this shift register by one `u64` and returns the bit mask.
    pub fn advance(&mut self) -> u64 {
        self.nth(63).unwrap();

        self.0
    }
}

impl iter::Iterator for LFSR {
    type Item = bool;

    /// Returns the next bit.
    fn next(&mut self) -> Option<Self::Item> {
        let bit = (self.0 ^ (self.0 >> 2) ^ (self.0 >> 3) ^ (self.0 >> 5)) & 1;

        self.0 = (self.0 >> 1) | (bit << 63);

        Some(bit != 0)
    }
}

impl Randomizer for LFSR {
    fn range(&mut self, a: usize, b: usize) -> usize {
        let val = self.advance() as usize;
        let (low, high) = if a < b { (a, b) } else { (b, a) };
        if low == high {
            low
        } else {
            low + val % (high - low)
        }
    }

    fn random(&mut self) -> f64 {
        self.advance() as f64 / u64::MAX as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that range works as advertised.
    #[test]
    fn lfsr_range() {
        let mut lfsr = LFSR::new(12345);
        for a in 0..100 {
            for b in a..a + 100 {
                for _ in 0..100 {
                    let v = lfsr.range(a, b);
                    if !(a <= v && v < b) {
                        println!("!({} <= {} < {})", a, v, b);
                    }
                    if b > a {
                        assert!(a <= v && v < b);
                    } else {
                        assert!(a == v && v == b);
                    }
                }
            }
        }
    }

    /// Tests that random gives a rectangular distribution.
    #[test]
    fn lfsr_random() {
        let mut lfsr = LFSR::new(12345);

        let buckets = 100;
        let iterations = 100 * 100 * buckets;
        let hist = (0..iterations).fold(vec![0; buckets], |mut hist, _| {
            hist[(buckets as f64 * lfsr.random()) as usize] += 1;
            hist
        });

        let mid = iterations / buckets;
        let h = 400;
        for v in hist {
            assert!(mid - h < v && v < mid + h);
        }
    }
}
