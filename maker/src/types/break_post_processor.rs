use std::str::FromStr;

use maze::initialize;

use crate::types::*;

/// A full description of the break action.
pub struct BreakPostProcessor {
    /// The heat map type.
    pub map_type: HeatMapType,

    /// The number of times to apply the operation.
    pub count: usize,
}

impl FromStr for BreakPostProcessor {
    type Err = String;

    /// Converts a string to a break description.
    ///
    /// The string can be on two forms:
    /// 1. `map_type`: If only a value that can be made into a
    ///    [HeatMapType](struct.HeatMapType.html) is passed, the `count` will be
    ///    `1`.
    /// 2. `map_type,count`: If a count is passed, it will be used as `count`.
    fn from_str(s: &str) -> Result<Self, String> {
        let mut parts = s.split(',').map(str::trim);
        let map_type =
            parts.next().map(|p| HeatMapType::from_str(p)).unwrap()?;

        if let Some(part1) = parts.next() {
            if let Ok(count) = usize::from_str_radix(part1, 10) {
                Ok(Self { map_type, count })
            } else {
                Err(format!("invalid count: {}", part1))
            }
        } else {
            Ok(Self { map_type, count: 1 })
        }
    }
}

impl<R> PostProcessor<R> for BreakPostProcessor
where
    R: initialize::Randomizer + Sized,
{
    /// Applies the break action.
    ///
    /// This action will repeatedly calculate a heat map, and then open walls in
    /// rooms with higher probability in hot rooms.
    ///
    /// # Arguments
    /// *  `maze` - The maze.
    /// *  `rng` - A random number generator.
    fn post_process(&self, mut maze: Maze, rng: &mut R) -> Maze {
        for _ in 0..self.count {
            let heat_map = self.map_type.generate(&maze);
            for pos in heat_map.positions() {
                if 1.0 / (rng.random() * f64::from(heat_map[pos])) < 0.5 {
                    loop {
                        let walls = maze.walls(pos);
                        let wall = walls[rng.range(0, walls.len())];
                        if maze.is_inside(maze.back((pos, wall)).0) {
                            maze.open((pos, wall));
                            break;
                        }
                    }
                }
            }
        }

        maze
    }
}
