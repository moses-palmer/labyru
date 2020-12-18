use std::str;

/// A colour.
#[derive(Clone, Copy, Default)]
pub struct Color {
    // The red component.
    pub red: u8,

    // The green component.
    pub green: u8,

    // The blue component.
    pub blue: u8,

    // The alpha component.
    pub alpha: u8,
}

impl Color {
    /// Returns a fully transparent version of this colour.
    pub fn transparent(self) -> Self {
        Self {
            red: self.red,
            green: self.blue,
            blue: self.blue,
            alpha: 0,
        }
    }

    /// Fades one colour to another.
    ///
    /// # Arguments
    /// *  `other` - The other colour.
    /// *  `w` - The weight of this colour. If this is `1.0` or greater, `self`
    ///   colour is returned; if this is 0.0 or less, `other` is returned;
    ///   otherwise a linear interpolation between the colours is returned.
    pub fn fade(self, other: Self, w: f32) -> Color {
        if w >= 1.0 {
            self
        } else if w <= 0.0 {
            other
        } else {
            let n = 1.0 - w;
            Color {
                red: (f32::from(self.red) * w + f32::from(other.red) * n) as u8,
                green: (f32::from(self.green) * w + f32::from(other.green) * n)
                    as u8,
                blue: (f32::from(self.blue) * w + f32::from(other.blue) * n)
                    as u8,
                alpha: (f32::from(self.alpha) * w + f32::from(other.alpha) * n)
                    as u8,
            }
        }
    }
}

impl str::FromStr for Color {
    type Err = String;

    /// Converts a string to a colour.
    ///
    /// This method supports colours on the form `#RRGGBB` and `#RRGGBBAA`,
    /// where `RR`, `GG`, `BB` and `AA` are the red, green, blue and alpha
    /// components hex encoded.
    ///
    /// # Arguments
    /// *  `s` - The string to convert.
    fn from_str(s: &str) -> Result<Color, String> {
        if !s.starts_with('#') || s.len() % 2 == 0 {
            Err(format!("unknown colour value: {}", s))
        } else {
            let data = s
                .bytes()
                // Skip the initial '#'
                .skip(1)
                // Hex decode and create list
                .map(|c| {
                    if (b'0'..=b'9').contains(&c) {
                        Some(c - b'0')
                    } else if (b'A'..=b'F').contains(&c) {
                        Some(c - b'A' + 10)
                    } else if (b'a'..=b'f').contains(&c) {
                        Some(c - b'a' + 10)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                // Join every byte
                .chunks(2)
                .map(|c| {
                    if let (Some(msb), Some(lsb)) = (c[0], c[1]) {
                        Some(msb << 4 | lsb)
                    } else {
                        None
                    }
                })
                // Ensure all values are valid
                .take_while(Option::is_some)
                .map(Option::unwrap)
                .collect::<Vec<_>>();

            match data.len() {
                3 => Ok(Color {
                    red: data[0],
                    green: data[1],
                    blue: data[2],
                    alpha: 255,
                }),
                4 => Ok(Color {
                    red: data[1],
                    green: data[2],
                    blue: data[3],
                    alpha: data[0],
                }),
                _ => Err(format!("invalid colour format: {}", s)),
            }
        }
    }
}

impl ToString for Color {
    /// Converts a colour to a string.
    ///
    /// This method ignores the alpha component.
    fn to_string(&self) -> String {
        format!("#{:02.X}{:02.X}{:02.X}", self.red, self.green, self.blue)
    }
}
