use rocket::http;
use rocket::request;

use labyru::initialize;

/// A random seed.
pub struct Seed {
    /// The LFSR initialised with the seed.
    lfsr: initialize::LFSR,
}

impl<'a> request::FromFormValue<'a> for Seed {
    type Error = &'a http::RawStr;

    fn from_form_value(
        form_value: &'a http::RawStr,
    ) -> Result<Self, Self::Error> {
        let seed = form_value.parse::<u64>().map_err(|_| form_value)?;
        let lfsr = initialize::LFSR::new(seed);
        Ok(Self { lfsr })
    }

    fn default() -> Option<Self> {
        Some(Self {
            lfsr: initialize::LFSR::new(rand::random()),
        })
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
