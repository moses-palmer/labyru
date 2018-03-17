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
