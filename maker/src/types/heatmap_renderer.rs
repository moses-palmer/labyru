use std::str::FromStr;

use svg::Node;

use maze_tools::image::Color;

use crate::types::*;

/// A full description of the heat map action.
#[derive(Clone)]
pub struct HeatMapRenderer {
    /// The heat map type.
    pub map_type: HeatMapType,

    /// The colour of cold regioins.
    pub from: Color,

    /// The colour of hot regions.
    pub to: Color,
}

impl FromStr for HeatMapRenderer {
    type Err = String;

    /// Converts a string to a heat map description.
    ///
    /// The string can be on three forms:
    /// 1. `map_type`: If only a value that can be made into a
    ///    [`HeatMapType`](HeatMapType) is passed, the `from` and `to` values
    ///    will be `#000000FF` and `#FFFF0000`.
    /// 2. `map_type,colour`: If only one colour is passed, the `from` and `to`
    ///    values will be `#00000000` and the colour passed.
    /// 3. `map_type,from,to`: If two colours are passed, they are used as
    ///    `from` and `to` values.
    fn from_str(s: &str) -> Result<Self, String> {
        let mut parts = s.split(',').map(str::trim);
        let map_type = parts.next().map(HeatMapType::from_str).unwrap()?;

        if let Some(part1) = parts.next() {
            if let Some(part2) = parts.next() {
                Ok(Self {
                    map_type,
                    from: Color::from_str(part1)?,
                    to: Color::from_str(part2)?,
                })
            } else {
                Ok(Self {
                    map_type,
                    from: Color::from_str(part1).map(Color::transparent)?,
                    to: Color::from_str(part1)?,
                })
            }
        } else {
            Ok(Self {
                map_type,
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

impl Renderer for HeatMapRenderer {
    /// Applies the heat map action.
    ///
    /// This action will calculate a heat map, and use the heat of each room to
    /// interpolate between the colours in `action`.
    ///
    /// # Arguments
    /// *  `maze` - The maze.
    /// *  `group` - The group to which to add the rooms.
    fn render(&self, maze: &Maze, group: &mut svg::node::element::Group) {
        let matrix = self.map_type.generate(maze);
        let max = *matrix.values().max().unwrap() as f32;
        group.append(draw_rooms(maze, |pos| {
            self.to.fade(self.from, matrix[pos] as f32 / max)
        }));
    }
}
