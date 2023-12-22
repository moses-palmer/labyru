use std::path::{Path, PathBuf};

use clap::{arg, Parser};
use svg::Node;

use maze::render::svg::ToPath;

mod types;
use self::types::*;

/// Generates mazes.
#[derive(Parser)]
#[command(author, version, about)]
struct Arguments {
    /// The number of walls per room: 3, 4 or 6.
    #[arg(
        id = "SHAPE",
        long = "walls",
        default_value = "4",
        value_parser = |s: &str| -> Result<maze::Shape, String> {
            s.parse::<u32>()
                .map_err(|_| format!("invalid number: {}", s))
                .and_then(|n| n.try_into()
                    .map_err(|e| format!("invalid number of walls: {}", e)))
        },
    )]
    shape: maze::Shape,

    /// The width of the maze, in rooms.
    #[arg(
        id = "WIDTH",
        long = "width",
        required_unless_present_all(["BACKGROUND", "RATIO"]),
    )]
    width: Option<usize>,

    /// The height of the maze, in rooms.
    #[arg(
        id = "HEIGHT",
        long = "height",
        required_unless_present_all(["BACKGROUND", "RATIO"]),
    )]
    height: Option<usize>,

    /// The initialisation methods to use.
    ///
    /// This is a comma separated list of the following values:
    ///
    /// braid: A maze containing loops.
    ///
    /// branching: A maze the frequently branches.
    ///
    /// winding: A maze with long corridors.
    ///
    /// clear: A clear area.
    #[arg(id = "METHOD", long = "method", required(true))]
    methods: Methods<Random>,

    /// A relative size for the maze, applied to rooms.
    #[arg(id = "SCALE", long = "scale", default_value_t = 10.0)]
    scale: f32,

    /// A seed for the random number generator.
    #[arg(id = "SEED", long = "seed")]
    seed: Option<u64>,

    /// The margin around the maze.
    #[arg(id = "MARGIN", long = "margin", default_value_t = 10.0)]
    margin: f32,

    /// A mask image to determine which rooms are part of the mask and
    /// thenshold luminosity value between 0 and 1 on the form "path,0.5".
    #[arg(id = "INITIALIZE", long = "mask")]
    initialize_mask: Option<MaskInitializer<Random>>,

    /// Whether to create a heat map.
    #[arg(id = "HEATMAP", long = "heat-map")]
    render_heatmap: Option<HeatMapRenderer>,

    /// A background image to colour rooms.
    #[arg(id = "BACKGROUND", long = "background")]
    render_background: Option<BackgroundRenderer>,

    /// A ratio for pixels per room when using a background.
    #[arg(
        id = "RATIO",
        long = "ratio",
        conflicts_with_all(["WIDTH", "HEIGHT"]),
        requires("BACKGROUND"),
    )]
    render_background_ratio: Option<f32>,

    /// A text to draw on the maze.
    #[arg(id = "TEXT", long = "text")]
    render_text: Option<TextRenderer>,

    /// Whether to solve the maze, and the solution colour. If not specified,
    /// the colour defaults to "black".
    #[arg(
        id = "SOLVE",
        long = "solve",
        default_missing_value = "black",
        conflicts_with_all(["INITIALIZE"]),
    )]
    render_solve: Option<SolveRenderer>,

    /// Whether to break the maze.
    #[arg(long = "break")]
    post_break: Option<BreakPostProcessor>,

    /// The output SVG.
    #[arg(id = "PATH", required(true))]
    output: PathBuf,
}

#[allow(unused_variables, clippy::too_many_arguments)]
fn run<P>(
    maze: Maze,
    scale: f32,
    margin: f32,
    renderers: &[&dyn Renderer],
    output: P,
) where
    P: AsRef<Path>,
{
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
    let args = Arguments::parse();

    // Parse maze information
    let (width, height) = args
        .render_background_ratio
        .and_then(|render_background_ratio| {
            println!("RENDER BACKGROUND RATIO {}", render_background_ratio);
            args.render_background.as_ref().map(|render_background| {
                args.shape.minimal_dimensions(
                    render_background.image.width() as f32
                        / render_background_ratio,
                    render_background.image.height() as f32
                        / render_background_ratio,
                )
            })
        })
        .unwrap_or_else(|| (args.width.unwrap(), args.height.unwrap()));

    let mut rng = args
        .seed
        .map(Random::from_seed)
        .unwrap_or_else(Random::from_os);

    // Make sure the maze is initialised
    let maze = {
        let mut maze = args.initialize_mask.initialize(
            args.shape.create(width, height),
            &mut rng,
            args.methods,
        );

        [&args.post_break as &dyn PostProcessor<_>]
            .iter()
            .fold(maze, |maze, a| a.post_process(maze, &mut rng))
    };

    run(
        maze,
        args.scale,
        args.margin,
        &[
            &args.render_background,
            &args.render_text,
            &args.render_heatmap,
            &args.render_solve,
        ],
        &args.output,
    );
}
