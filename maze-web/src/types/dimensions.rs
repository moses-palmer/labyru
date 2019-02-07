use rocket::http;
use rocket::request;

/// Dimensions of a maze.
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
