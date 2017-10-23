use types::*;


/// A full description of the heat map action.
pub struct HeatMapAction {
    /// The heat map type.
    pub map_type: HeatMapType,

    /// The colour of cold regioins.
    pub from: Color,

    /// The colour of hot regions.
    pub to: Color,
}


impl HeatMapAction {
    /// Converts a string to a heat map description.
    ///
    /// The string can be on three forms:
    /// 1. `map_type`: If only a value that can be made into a
    ///    [HeatMapType](struct.HeatMapType.html) is passed, the `from` and `to`
    ///    values will be `#000000FF` and `#FFFF0000`.
    /// 2. `map_type,colour`: If only one colour is passed, the `from` and `to`
    ///    values will be `#00000000` and the colour passed.
    /// 3. `map_type,from,to`: If two colours are passed, they are used as
    ///    `from` and `to` values.
    pub fn from_str(s: &str) -> Result<Self, String> {
        let mut parts = s.split(",").map(|p| p.trim());
        let map_type = parts.next().map(|p| HeatMapType::from_str(p)).unwrap()?;

        if let Some(part1) = parts.next() {
            if let Some(part2) = parts.next() {
                Ok(Self {
                    map_type: map_type,
                    from: Color::from_str(part1)?,
                    to: Color::from_str(part2)?,
                })
            } else {
                Ok(Self {
                    map_type: map_type,
                    from: Color::from_str(part1).map(|c| c.transparent())?,
                    to: Color::from_str(part1)?,
                })
            }
        } else {
            Ok(Self {
                map_type: map_type,
                from: Color {
                    red: 0,
                    green: 0,
                    blue: 255,
                    alpha: 0,
                },
                to: Color {
                    red: 255,
                    green: 0,
                    blue: 0,
                    alpha: 255,
                },
            })
        }
    }
}
