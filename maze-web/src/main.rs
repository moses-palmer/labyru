#![feature(proc_macro_hygiene, decl_macro)]

extern crate rand;
#[macro_use]
extern crate rocket;
extern crate svg;

extern crate maze;

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
        let mut maze: Box<maze::Maze> =
            source.maze_type.create(source.dimensions);
        maze.randomized_prim(&mut source.seed);

        let mut container = svg::node::element::Group::new();
        container
            .append(svg::node::element::Path::new().set("d", maze.to_path_d()));
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

#[get("/<maze_type>/<dimensions>/image.svg?<seed>")]
fn maze_svg<'a>(
    maze_type: types::MazeType,
    dimensions: types::Dimensions,
    seed: types::Seed,
) -> Maze {
    Maze {
        maze_type,
        dimensions,
        seed,
    }
    .into()
}

fn main() {
    rocket::ignite().mount("/", routes![maze_svg]).launch();
}
