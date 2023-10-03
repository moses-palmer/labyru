use std::convert::TryInto;
use std::f32;

use clap::{
    crate_authors, crate_version, value_parser, Arg, ArgAction, Command,
};
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
    (maze.viewbox() * scale).expand(margin).tuple()
}

#[allow(unused_mut)]
fn main() {
    let mut args = Command::new("")
        .about("Generates mazes")
        .version(crate_version!())
        .author(crate_authors!(", "))
        .arg(
            Arg::new("WALLS")
                .long("walls")
                .default_value("4")
                .value_parser(value_parser!(u32))
                .help("The number of walls per room; 3, 4 or 6."),
        )
        .arg(
            Arg::new("WIDTH")
                .long("width")
                .required_unless_present_all(["BACKGROUND", "RATIO"])
                .value_parser(value_parser!(usize))
                .help("The width of the maze, in rooms."),
        )
        .arg(
            Arg::new("HEIGHT")
                .long("height")
                .required_unless_present_all(["BACKGROUND", "RATIO"])
                .value_parser(value_parser!(usize))
                .help("The height of the maze, in rooms."),
        )
        .arg(
            Arg::new("METHOD")
                .long("method")
                .value_parser(value_parser!(Methods<rand::rngs::OsRng>))
                .help("The initialisation method to use."),
        )
        .arg(
            Arg::new("SCALE")
                .long("scale")
                .help("A relative size for the maze, applied to rooms."),
        )
        .arg(
            Arg::new("MARGIN")
                .long("margin")
                .help("The margin around the maze."),
        )
        .arg(
            Arg::new("SOLVE")
                .long("solve")
                .action(ArgAction::SetTrue)
                .help("Whether to solve the maze."),
        )
        .arg(
            Arg::new("BREAK")
                .long("break")
                .value_parser(value_parser!(BreakPostProcessor))
                .help("Whether to break the maze."),
        )
        .arg(
            Arg::new("HEATMAP")
                .long("heat-map")
                .value_parser(value_parser!(HeatMapRenderer))
                .help("Whether to create a heat map."),
        )
        .arg(Arg::new("OUTPUT").help("The output file name."))
        .arg(
            Arg::new("BACKGROUND")
                .long("background")
                .value_parser(value_parser!(BackgroundRenderer))
                .help("A background image to colour rooms."),
        )
        .arg(
            Arg::new("TEXT")
                .long("text")
                .value_parser(value_parser!(TextRenderer))
                .help("A text to draw on the maze."),
        )
        .arg(
            Arg::new("MASK")
                .long("mask")
                .value_parser(value_parser!(MaskInitializer<rand::rngs::OsRng>))
                .help("A background image to colour rooms."),
        )
        .arg(
            Arg::new("RATIO")
                .long("ratio")
                .help("A ratio for pixels per room when using a background.")
                .conflicts_with_all(["WIDTH", "HEIGHT"])
                .requires("BACKGROUND"),
        )
        .get_matches();

    // Parse general rendering options
    let scale = args.remove_one::<f32>("SCALE").unwrap_or(10.0);
    let margin = args.remove_one::<f32>("MARGIN").unwrap_or(10.0);

    // Parse initialisers
    let initializers: Methods<_> =
        args.remove_one::<Methods<_>>("METHOD").unwrap_or_default();
    let mask_initializer: Option<MaskInitializer<_>> =
        args.remove_one::<MaskInitializer<_>>("MASK");

    // Parse post-processors
    let break_post_processor: Option<BreakPostProcessor> =
        args.remove_one::<BreakPostProcessor>("BREAK");

    // Parse renderers
    let heatmap_renderer: Option<HeatMapRenderer> =
        args.remove_one::<HeatMapRenderer>("HEATMAP");
    let background_renderer: Option<BackgroundRenderer> =
        args.remove_one::<BackgroundRenderer>("BACKGROUND");
    let text_renderer: Option<TextRenderer> =
        args.remove_one::<TextRenderer>("TEXT");
    let solve_renderer = if args.contains_id("SOLVE") {
        Some(SolveRenderer)
    } else {
        None
    };

    // Parse maze information
    let shape: maze::Shape = args
        .remove_one::<u32>("WALLS")
        .unwrap()
        .try_into()
        .expect("unknown number of walls");
    let (width, height) = args
        .get_one::<f32>("RATIO")
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
                args.remove_one::<usize>("WIDTH").unwrap(),
                args.remove_one::<usize>("HEIGHT").unwrap(),
            )
        });

    let output = args.remove_one::<String>("OUTPUT").unwrap();

    // Make sure the maze is initialised
    let mut rng = rand::rngs::OsRng;
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
        &output,
    );
}
