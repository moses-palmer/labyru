#[macro_use]
extern crate clap;
extern crate image;
extern crate rand;
extern crate rayon;
extern crate svg;

extern crate maze;

use std::f32;

use clap::{App, Arg};

use svg::Node;

use maze::prelude::*;

mod types;
use self::types::*;

#[allow(unused_variables, clippy::too_many_arguments)]
fn run(
    maze: &mut maze::Maze,
    scale: f32,
    margin: f32,
    solve: bool,
    break_action: Option<BreakAction>,
    heat_map_action: Option<HeatMapAction>,
    background_action: Option<BackgroundAction>,
    initialize_action: Option<InitializeAction>,
    output: &str,
) {
    let document = svg::Document::new()
        .set("viewBox", maze_to_viewbox(maze, scale, margin));
    let mut container = svg::node::element::Group::new()
        .set("transform", format!("scale({})", scale));

    // Make sure the maze is initialised
    if let Some(initialize_action) = initialize_action {
        initialize_action.apply(maze, &mut container);
    } else {
        maze.randomized_prim(&mut rand::weak_rng());
    }

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

    // Draw the solution
    if solve {
        container.append(
            svg::node::element::Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-linecap", "round")
                .set("stroke-linejoin", "round")
                .set("stroke-width", 0.4)
                .set("vector-effect", "non-scaling-stroke")
                .set(
                    "d",
                    maze.walk(
                        maze::matrix::Pos { col: 0, row: 0 },
                        maze::matrix::Pos {
                            col: maze.width() as isize - 1,
                            row: maze.height() as isize - 1,
                        },
                    )
                    .unwrap()
                    .to_path_d(),
                ),
        );
    }

    svg::save(output, &document.add(container)).expect("failed to write SVG");
}

/// Calculates the view box for a maze with a margin.
///
/// # Arguments
/// * `maze` - The maze for which to generate a view box.
/// * `scale` - A scale multiplier.
/// * `margin` - The margin to apply to all sides.
fn maze_to_viewbox(
    maze: &maze::Maze,
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
    let mut app = App::new("")
        .about("Generates mazes")
        .version(crate_version!())
        .author(crate_authors!(", "))
        .arg(
            Arg::with_name("WALLS")
                .long("--walls")
                .takes_value(true)
                .default_value("4")
                .help("The number of walls per room; 3, 4 or 6."),
        )
        .arg(
            Arg::with_name("WIDTH")
                .long("--width")
                .takes_value(true)
                .default_value("12")
                .help("The width of the maze, in rooms."),
        )
        .arg(
            Arg::with_name("HEIGHT")
                .long("--height")
                .takes_value(true)
                .default_value("9")
                .help("The height of the maze, in rooms."),
        )
        .arg(
            Arg::with_name("SCALE")
                .long("--scale")
                .takes_value(true)
                .help("A relative size for the maze, applied to rooms."),
        )
        .arg(
            Arg::with_name("MARGIN")
                .long("--margin")
                .takes_value(true)
                .help("The margin around the maze."),
        )
        .arg(
            Arg::with_name("SOLVE")
                .long("--solve")
                .takes_value(false)
                .help("Whether to solve the maze."),
        )
        .arg(
            Arg::with_name("BREAK")
                .long("--break")
                .takes_value(true)
                .help("Whether to break the maze."),
        )
        .arg(
            Arg::with_name("HEATMAP")
                .long("--heat-map")
                .takes_value(true)
                .help("Whether to create a heat map."),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .required(true)
                .help("The output file name."),
        );

    #[cfg(feature = "background")]
    {
        app = app.arg(
            Arg::with_name("BACKGROUND")
                .long("background")
                .help("A background image to colour rooms.")
                .takes_value(true),
        );
    }

    let args = app.get_matches();

    let mut maze = maze::MazeType::from_num(
        args.value_of("WALLS")
            .map(|s| s.parse().expect("invalid wall value"))
            .unwrap(),
    )
    .expect("unknown number of walls")
    .create(
        args.value_of("WIDTH")
            .map(|s| s.parse().expect("invalid width"))
            .unwrap(),
        args.value_of("HEIGHT")
            .map(|s| s.parse().expect("invalid height"))
            .unwrap(),
    );

    run(
        maze.as_mut(),
        args.value_of("SCALE")
            .map(|s| s.parse().expect("invalid scale"))
            .unwrap_or(10.0),
        args.value_of("MARGIN")
            .map(|s| s.parse().expect("invalid margin"))
            .unwrap_or(10.0),
        args.is_present("SOLVE"),
        args.value_of("BREAK")
            .map(|s| s.parse().expect("invalid break")),
        args.value_of("HEATMAP")
            .map(|s| s.parse().expect("invalid heat map")),
        args.value_of("BACKGROUND")
            .map(|s| s.parse().expect("invalid background")),
        args.value_of("MASK")
            .map(|s| s.parse().expect("invalid mask")),
        args.value_of("OUTPUT").unwrap(),
    );
}
