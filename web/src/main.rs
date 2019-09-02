#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};

mod types;

#[get("/<maze_type>/<dimensions>/image.svg?<seed>&<solve>")]
fn maze_svg(
    maze_type: types::MazeType,
    dimensions: types::Dimensions,
    seed: types::Seed,
    solve: bool,
) -> types::Maze {
    types::Maze {
        maze_type,
        dimensions,
        seed,
        solve,
    }
}

fn main() {
    rocket::ignite().mount("/", routes![maze_svg]).launch();
}
