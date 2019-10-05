use std::convert::TryInto;
use std::f32;

use clap::{crate_authors, crate_version, App, Arg};
use svg::Node;

use maze::render::svg::ToPath;

mod types;
use self::types::*;

#[allow(unused_variables, clippy::too_many_arguments)]
fn run(
    maze: Maze,
    scale: f32,
    margin: f32,
    renderers: &[&dyn Renderer],
    output: &str,
) {
    let document = svg::Document::new()
        .set("viewBox", maze_to_viewbox(&maze, scale, margin));
    let mut container = svg::node::element::Group::new()
        .set("transform", format!("scale({})", scale));

    for renderer in renderers {
        renderer.render(&maze, &mut container);
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
/// *  `maze` - The maze for which to generate a view box.
/// *  `scale` - A scale multiplier.
/// *  `margin` - The margin to apply to all sides.
fn maze_to_viewbox(
    maze: &Maze,
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
                .required_unless_all(&["BACKGROUND", "RATIO"])
                .help("The width of the maze, in rooms."),
        )
        .arg(
            Arg::with_name("HEIGHT")
                .long("--height")
                .takes_value(true)
                .required_unless_all(&["BACKGROUND", "RATIO"])
                .help("The height of the maze, in rooms."),
        )
        .arg(
            Arg::with_name("METHOD")
                .long("--method")
                .takes_value(true)
                .help("The initialisation method to use."),
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
        )
        .arg(
            Arg::with_name("BACKGROUND")
                .long("background")
                .help("A background image to colour rooms.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("TEXT")
                .long("text")
                .help("A text to draw on the maze.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("MASK")
                .long("mask")
                .help("A background image to colour rooms.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("RATIO")
                .long("ratio")
                .help("A ratio for pixels per room when using a background.")
                .conflicts_with_all(&["WIDTH", "HEIGHT"])
                .requires("BACKGROUND")
                .takes_value(true),
        );

    let args = app.get_matches();

    // Parse general rendering options
    let scale = args
        .value_of("SCALE")
        .map(|s| s.parse().expect("invalid scale"))
        .unwrap_or(10.0);
    let margin = args
        .value_of("MARGIN")
        .map(|s| s.parse().expect("invalid margin"))
        .unwrap_or(10.0);

    // Parse initialisers
    let initializers: Methods<_> = args
        .value_of("METHOD")
        .map(str::parse)
        .unwrap_or_else(|| Ok(Methods::default()))
        .expect("invalid initialisation methods");
    let mask_initializer: Option<MaskInitializer<_>> = args
        .value_of("MASK")
        .map(|s| s.parse().expect("invalid mask"));

    // Parse post-processors
    let break_post_processor: Option<BreakPostProcessor> = args
        .value_of("BREAK")
        .map(|s| s.parse().expect("invalid break"));

    // Parse renderers
    let heatmap_renderer: Option<HeatMapRenderer> = args
        .value_of("HEATMAP")
        .map(|s| s.parse().expect("invalid heat map"));
    let background_renderer: Option<BackgroundRenderer> = args
        .value_of("BACKGROUND")
        .map(|s| s.parse().expect("invalid background"));
    let text_renderer: Option<TextRenderer> = args
        .value_of("TEXT")
        .map(|s| s.parse().expect("invalid text"));
    let solve_renderer = if args.is_present("SOLVE") {
        Some(SolveRenderer)
    } else {
        None
    };

    // Parse maze information
    let shape: maze::Shape = args
        .value_of("WALLS")
        .map(|s| s.parse::<u32>().expect("invalid wall value"))
        .unwrap()
        .try_into()
        .expect("unknown number of walls");
    let (width, height) = args
        .value_of("RATIO")
        .map(|s| s.parse::<f32>().expect("invalid ratio"))
        .and_then(|ratio| {
            background_renderer.as_ref().map(|background_renderer| {
                shape.minimal_dimensions(
                    background_renderer.image.width() as f32 / ratio,
                    background_renderer.image.height() as f32 / ratio,
                )
            })
        })
        .unwrap_or_else(|| {
            (
                args.value_of("WIDTH")
                    .map(|s| s.parse().expect("invalid width"))
                    .unwrap(),
                args.value_of("HEIGHT")
                    .map(|s| s.parse().expect("invalid height"))
                    .unwrap(),
            )
        });

    let output = args.value_of("OUTPUT").unwrap();

    // Make sure the maze is initialised
    let mut rng = rand::weak_rng();
    let maze = {
        let mut maze = mask_initializer.initialize(
            shape.create(width, height),
            &mut rng,
            initializers,
        );

        [&break_post_processor as &dyn PostProcessor<_>]
            .iter()
            .fold(maze, |maze, a| a.post_process(maze, &mut rng))
    };

    run(
        maze,
        scale,
        margin,
        &[
            &background_renderer,
            &text_renderer,
            &heatmap_renderer,
            &solve_renderer,
        ],
        output,
    );
}
