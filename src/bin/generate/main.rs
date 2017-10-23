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

use rand::Rng;

use svg::Document;
use svg::Node;
use svg::node::element::{Group, Path};
use svg::node::element::path::{Command, Data, Position};

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

    let document =
        Document::new().set("viewBox", maze_to_viewbox(maze, scale, margin));
    let mut container =
        Group::new().set("transform", format!("scale({})", scale));

    if let Some(background_action) = background_action {
        #[cfg(feature = "background")]
        apply_background(background_action, maze, &mut container);
    }

    if let Some(break_action) = break_action {
        apply_break(break_action, maze, &mut container);
    }

    if let Some(heat_map_action) = heat_map_action {
        apply_heat_map(heat_map_action, maze, &mut container);
    }

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


/// Applies the background action.
///
/// This action will use an image to sample the background colour of rooms.
///
/// # Arguments
/// * `action` - The action parameters.
/// * `maze` - The maze.
/// * `group` - The group to which to add the rooms.
#[cfg(feature = "background")]
fn apply_background(
    action: types::background_action::BackgroundAction,
    maze: &mut labyru::Maze,
    group: &mut Group,
) {
    let (left, top, width, height) = maze.viewbox();
    let rgb = image::open(action.path.as_path())
        .expect("unable to open background image")
        .to_rgb();
    let (cols, rows) = rgb.dimensions();
    let data = rgb
        .enumerate_pixels()

        // Add all pixels inside a room to the cell representing the room
        .fold(
            labyru::matrix::Matrix::<(u32, (u32, u32, u32))>::new(
                maze.width(), maze.height()),
            |mut matrix, (x, y, pixel)| {
                let physical_pos = (
                    left + width * (x as f32 / cols as f32),
                    top + height * (y as f32 / rows as f32),
                );
                let pos = maze.room_at(physical_pos);
                if maze.rooms().is_inside(pos) {
                    matrix[pos] = (
                        matrix[pos].0 + 1, (
                            (matrix[pos].1).0 + pixel[0] as u32,
                            (matrix[pos].1).1 + pixel[1] as u32,
                            (matrix[pos].1).2 + pixel[2] as u32,
                        ));
                }

                matrix
            }
        )

        // Convert the summed colour values to an actual colour
        .map(
            |value| {
                let (count, pixel) = value;
                types::Color {
                    red: (pixel.0 / (count + 1)) as u8,
                    green: (pixel.1 / (count + 1)) as u8,
                    blue: (pixel.2 / (count + 1)) as u8,
                    alpha: 255,
                }
            }
        );

    group.append(draw_rooms(maze, |pos| data[pos]));
}


/// Applies the break action.
///
/// This action will repeatedly calculate a heat map, and then open walls in
/// rooms with higher probability in hot rooms.
///
/// # Arguments
/// * `action` - The action parameters.
/// * `maze` - The maze.
fn apply_break(
    action: types::break_action::BreakAction,
    maze: &mut labyru::Maze,
    _: &mut Group,
) {
    let mut rng = rand::weak_rng();

    for _ in 0..action.count {
        let heat_map = action.map_type.generate(maze);
        for pos in heat_map.positions() {
            if 1.0 / (rng.next_f32() * heat_map[pos] as f32) < 0.5 {
                loop {
                    let wall = rng.choose(maze.walls(pos)).unwrap();
                    if maze.rooms().is_inside(maze.back((pos, wall)).0) {
                        maze.open((pos, wall));
                        break;
                    }
                }
            }
        }
    }
}


/// Applies the heat map action.
///
/// This action will calculate a heat map, and use the heat of each room to
/// interpolate between the colours in `action`.
///
/// # Arguments
/// * `action` - The action parameters.
/// * `maze` - The maze.
/// * `group` - The group to which to add the rooms.
fn apply_heat_map(
    action: types::heatmap_action::HeatMapAction,
    maze: &mut labyru::Maze,
    group: &mut Group,
) {
    let matrix = action.map_type.generate(maze);
    let max = matrix.values().max().unwrap() as f32;
    group.append(draw_rooms(maze, |pos| {
        action.to.fade(&action.from, matrix[pos] as f32 / max)
    }));
}


/// Draws all rooms of a maze.
///
/// # Arguments
/// * `maze` - The maze to draw.
/// * `colors` - A function determining the colour of a room.
fn draw_rooms<F>(maze: &labyru::Maze, colors: F) -> svg::node::element::Group
where
    F: Fn(labyru::matrix::Pos) -> types::Color,
{
    let mut group = svg::node::element::Group::new();
    for pos in maze.rooms().positions().filter(
        |pos| maze.rooms()[*pos].visited,
    )
    {
        let color = colors(pos);
        let mut commands = maze.walls(pos)
            .iter()
            .enumerate()
            .map(|(i, wall)| {
                let (coords, _) = maze.corners((pos, wall));
                if i == 0 {
                    Command::Move(Position::Absolute, coords.into())
                } else {
                    Command::Line(Position::Absolute, coords.into())
                }
            })
            .collect::<Vec<_>>();
        commands.push(Command::Close);

        group.append(
            svg::node::element::Path::new()
                .set("fill", color.to_string())
                .set("fill-opacity", (color.alpha as f32 / 255.0))
                .set("d", Data::from(commands)),
        );
    }

    group
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
