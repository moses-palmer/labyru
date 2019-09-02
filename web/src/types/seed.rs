use rocket::http;
use rocket::request;
use serde::Deserialize;

use maze::initialize;

/// A random seed.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(transparent)]
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

#[cfg(test)]
mod tests {
    use serde_urlencoded;

    use super::*;

    #[test]
    fn deserialize() {
        assert_eq!(
            Seed {
                lfsr: initialize::LFSR::new(1234)
            },
            serde_urlencoded::from_str::<Vec<(String, Seed)>>(&"seed=1234")
                .unwrap()[0]
                .1,
        );
    }
}
