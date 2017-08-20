#[macro_use]
extern crate clap;
extern crate rand;
extern crate svg;

extern crate labyru;

use std::f32;
use std::str::FromStr;

use svg::Document;
use svg::Node;
use svg::node::element::{Group, Path};
use svg::node::element::path::{Command, Data, Position};

use labyru::initialize::randomized_prim::*;
use labyru::renderable::svg::*;



fn run(
    maze: &mut labyru::Maze,
    scale: f32,
    margin: f32,
    output: &str,
) {
    // Make sure the maze is initialised
    maze.randomized_prim(&mut rand::weak_rng());

    let document =
        Document::new().set("viewBox", maze_to_viewbox(maze, scale, margin));
    let mut container =
        Group::new().set("transform", format!("scale({})", scale));

    // Draw the maze
    container.append(
        Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-linecap", "round")
            .set("stroke-linejoin", "round")
            .set("stroke-width", 0.4)
            .set("vector-effect", "non-scaling-stroke")
            .set("d", maze.to_path_d()),
    );

    svg::save(output, &document.add(container)).expect("failed to write SVG");
}


/// Calculates the view box for a maze with a margin.
///
/// # Arguments
/// * `maze` - The maze for which to generate a view box.
/// * `scale` - A scale multiplier.
/// * `margin` - The margin to apply to all sides.
fn maze_to_viewbox(
    maze: &labyru::Maze,
    scale: f32,
    margin: f32,
) -> (f32, f32, f32, f32) {
    let viewbox = maze.viewbox();

    (
        viewbox.0 * scale - margin,
        viewbox.1 * scale - margin,
        viewbox.2 * scale + 2.0 * margin,
        viewbox.3 * scale + 2.0 * margin,
    )
}


fn main() {
    let args = clap_app!(myapp =>
        (about: "Generates mazes")
        (version: crate_version!())
        (author: crate_authors!(", "))

        (@arg WALLS:
            --("walls")
            +takes_value
            "The number of walls per room; 3, 4 or 6.")

        (@arg WIDTH:
            --("width")
            +takes_value
            "The width of the maze, in rooms.")

        (@arg HEIGHT:
            --("height")
            +takes_value
            "The height of the maze, in rooms.")

        (@arg SCALE:
            --("scale")
            +takes_value
            "A relative size for the maze, applied to rooms.")

        (@arg MARGIN:
            --("margin")
            +takes_value
            "The margin around the maze.")

        (@arg OUTPUT:
            +required
            "The output file name.")
    ).get_matches();

    let maze_type = labyru::MazeType::from_num(
        args.value_of("WALLS")
            .map(|s| u32::from_str_radix(s, 10).expect("invalid wall value"))
            .unwrap_or(4),
    ).expect("unknown number of walls");
    let mut maze = maze_type.create(
        args.value_of("WIDTH")
            .map(|s| usize::from_str_radix(s, 10).expect("invalid width"))
            .unwrap_or(12usize),
        args.value_of("HEIGHT")
            .map(|s| usize::from_str_radix(s, 10).expect("invalid height"))
            .unwrap_or(9usize),
    );

    let scale = args.value_of("SCALE")
        .map(|s| f32::from_str(s).expect("invalid scale"))
        .unwrap_or(10.0);

    let margin = args.value_of("MARGIN")
        .map(|s| f32::from_str(s).expect("invalid margin"))
        .unwrap_or(10.0);

    let output = args.value_of("OUTPUT").unwrap();

    run(
        maze.as_mut(),
        scale,
        margin,
        output,
    );
}
