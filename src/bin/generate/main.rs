#[macro_use]
extern crate clap;
extern crate rand;
extern crate svg;

extern crate labyru;

#[cfg(feature = "background")]
extern crate image;

#[cfg(feature = "parallel")]
extern crate rayon;

use std::f32;
use std::str::FromStr;

use svg::Node;

use labyru::initialize::randomized_prim::*;
use labyru::renderable::svg::*;

mod types;
use types::Action;


#[allow(unused_variables)]
fn run(
    maze: &mut labyru::Maze,
    scale: f32,
    margin: f32,
    break_action: Option<types::break_action::BreakAction>,
    heat_map_action: Option<types::heatmap_action::HeatMapAction>,
    background_action: Option<types::background_action::BackgroundAction>,
    output: &str,
) {
    // Make sure the maze is initialised
    maze.randomized_prim(&mut rand::weak_rng());

    let document = svg::Document::new().set(
        "viewBox",
        maze_to_viewbox(maze, scale, margin)
    );
    let mut container = svg::node::element::Group::new().set(
        "transform",
        format!("scale({})", scale)
    );

    if let Some(background_action) = background_action {
        background_action.apply(maze, &mut container);
    }

    if let Some(break_action) = break_action {
        break_action.apply(maze, &mut container);
    }

    if let Some(heat_map_action) = heat_map_action {
        heat_map_action.apply(maze, &mut container);
    }

    // Draw the maze
    container.append(
        svg::node::element::Path::new()
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


#[allow(unused_mut)]
fn main() {
    let mut app = clap_app!(myapp =>
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

        (@arg BREAK:
            --("break")
            +takes_value
            "Whether to break the maze.")

        (@arg HEATMAP:
            --("heat-map")
            +takes_value
            "Whether to create a heat map.")

        (@arg OUTPUT:
            +required
            "The output file name.")
    );

    #[cfg(feature = "background")]
    {
        app = app.arg(clap::Arg::with_name("BACKGROUND")
            .long("background")
            .help("A background image to colour rooms.")
            .takes_value(true));
    }

    let args = app.get_matches();

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

    let break_action = args.value_of("BREAK").map(|s| {
        types::break_action::BreakAction::from_str(s)
            .expect("invalid break")
    });

    let heat_map_action = args.value_of("HEATMAP").map(|s| {
        types::heatmap_action::HeatMapAction::from_str(s)
            .expect("invalid heat map")
    });

    let background_action = args.value_of("BACKGROUND").map(|s| {
        types::background_action::BackgroundAction::from_str(s)
            .expect("invalid background")
    });

    let output = args.value_of("OUTPUT").unwrap();

    run(
        maze.as_mut(),
        scale,
        margin,
        break_action,
        heat_map_action,
        background_action,
        output,
    );
}
