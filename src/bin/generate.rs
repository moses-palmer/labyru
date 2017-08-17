#[macro_use]
extern crate clap;
extern crate rand;
extern crate svg;

extern crate labyru;

use std::f32;
use std::str::FromStr;

use labyru::initialize::randomized_prim::*;
use labyru::renderable::svg::*;


fn run(maze: &mut labyru::Maze, scale: f32, margin: f32, output: &str) {
    svg::save(
        output,
        &svg::Document::new()
            .set("viewBox", maze_to_viewbox(maze, scale, margin))
            .add(
                svg::node::element::Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-linecap", "round")
                    .set("stroke-linejoin", "round")
                    .set("stroke-width", 0.4)
                    .set("transform", format!("scale({})", scale))
                    .set("vector-effect", "non-scaling-stroke")
                    .set(
                        "d",
                        maze.randomized_prim(&mut rand::weak_rng()).to_path_d(),
                    ),
            ),
    ).expect("failed to write SVG");
}


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

    let mut maze = labyru::shape::hex::Maze::new(
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

    run(&mut maze, scale, margin, output);
}
