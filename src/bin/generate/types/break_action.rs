use types::*;


/// A full description of the break action.
pub struct BreakAction {
    /// The heat map type.
    pub map_type: HeatMapType,

    /// The number of times to apply the operation.
    pub count: usize,
}


impl Action for BreakAction {
    /// Converts a string to a break description.
    ///
    /// The string can be on two forms:
    /// 1. `map_type`: If only a value that can be made into a
    ///    [HeatMapType](struct.HeatMapType.html) is passed, the `count` will be
    ///    `1`.
    /// 2. `map_type,count`: If a count is passed, it will be used as `count`.
    fn from_str(s: &str) -> Result<Self, String> {
        let mut parts = s.split(",").map(|p| p.trim());
        let map_type = parts.next().map(|p| HeatMapType::from_str(p)).unwrap()?;

        if let Some(part1) = parts.next() {
            if let Ok(count) = usize::from_str_radix(part1, 10) {
                Ok(Self {
                    map_type: map_type,
                    count: count,
                })
            } else {
                Err(format!("invalid count: {}", part1))
            }
        } else {
            Ok(Self {
                map_type: map_type,
                count: 1,
            })
        }
    }
}
