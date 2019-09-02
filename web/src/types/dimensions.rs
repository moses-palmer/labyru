use std::convert::TryFrom;

use rocket::http;
use rocket::request;
use serde::Deserialize;

/// Dimensions of a maze.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(try_from = "String")]
pub struct Dimensions {
    /// The width.
    pub width: usize,

    /// The height.
    pub height: usize,
}

impl<'a> request::FromParam<'a> for Dimensions {
    type Error = &'a http::RawStr;

    fn from_param(form_value: &'a http::RawStr) -> Result<Self, Self::Error> {
        let mut parts = form_value.split('x');
        let width = parts
            .next()
            .ok_or(form_value)?
            .parse::<usize>()
            .map_err(|_| form_value)?;
        let height = parts
            .next()
            .ok_or(form_value)?
            .parse::<usize>()
            .map_err(|_| form_value)?;
        Ok(Self { width, height })
    }
}

impl TryFrom<String> for Dimensions {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.split('x');
        let width = parts
            .next()
            .unwrap()
            .parse::<usize>()
            .map_err(|_| String::from("invalid width"))?;
        let height = parts
            .next()
            .ok_or_else(|| String::from("no height specified"))?
            .parse::<usize>()
            .map_err(|_| String::from("invalid height"))?;
        Ok(Self { width, height })
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn deserialize() {
        assert_eq!(
            Dimensions {
                width: 1,
                height: 2,
            },
            String::from("1x2").try_into().unwrap(),
        );
        assert_eq!(
            Err(String::from("no height specified")),
            Dimensions::try_from(String::from("1")),
        );
        assert_eq!(
            Err(String::from("invalid width")),
            Dimensions::try_from(String::from("ax2")),
        );
        assert_eq!(
            Err(String::from("invalid height")),
            Dimensions::try_from(String::from("1xb")),
        );
    }
}
