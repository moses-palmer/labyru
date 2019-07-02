#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};

use std::io;

use rocket::http;
use rocket::response;
use svg::Node;

use maze::prelude::*;

mod types;

/// The maximum nmber of rooms.
const MAX_ROOMS: usize = 1000;

/// A responder providing an image of a maze.
struct Maze {
    maze_type: types::MazeType,
    dimensions: types::Dimensions,
    seed: types::Seed,
    solve: bool,
}

impl<'a> response::Responder<'a> for Maze {
    fn respond_to(self, _request: &rocket::Request) -> response::Result<'a> {
        let room_count = self.dimensions.width * self.dimensions.height;
        if room_count > MAX_ROOMS {
            rocket::Response::build()
                .status(http::Status::InsufficientStorage)
                .ok()
        } else {
            self.into()
        }
    }
}

impl<'a> From<Maze> for response::Result<'a> {
    fn from(mut source: Maze) -> Self {
        let maze = source
            .maze_type
            .create(source.dimensions)
            .randomized_prim(&mut source.seed);

        let mut container = svg::node::element::Group::new();
        container.append(
            svg::node::element::Path::new()
                .set("class", "walls")
                .set("d", maze.to_path_d()),
        );
        if source.solve {
            container.append(
                svg::node::element::Path::new().set("class", "path").set(
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
        let data = svg::Document::new()
            .set("viewBox", maze.viewbox())
            .add(container)
            .to_string();
        rocket::Response::build()
            .sized_body(io::Cursor::new(data))
            .header(http::ContentType::SVG)
            .ok()
    }
}

#[get("/<maze_type>/<dimensions>/image.svg?<seed>&<solve>")]
fn maze_svg(
    maze_type: types::MazeType,
    dimensions: types::Dimensions,
    seed: types::Seed,
    solve: bool,
) -> Maze {
    Maze {
        maze_type,
        dimensions,
        seed,
        solve,
    }
}

fn main() {
    rocket::ignite().mount("/", routes![maze_svg]).launch();
}
