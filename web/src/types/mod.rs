use actix_web::{error, Error, HttpRequest, HttpResponse, Responder};
use svg::Node;

use maze::initialize;
use maze::render::svg::ToPath;

mod maze_type;
pub use self::maze_type::*;
mod dimensions;
pub use self::dimensions::*;
mod seed;
pub use self::seed::*;

/// The maximum nmber of rooms.
const MAX_ROOMS: usize = 1000;

/// A responder providing an image of a maze.
pub struct Maze {
    pub maze_type: MazeType,
    pub dimensions: Dimensions,
    pub seed: Seed,
    pub solve: bool,
}

impl Responder for Maze {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let room_count = self.dimensions.width * self.dimensions.height;
        if room_count > MAX_ROOMS {
            Err(error::ErrorInsufficientStorage(
                "the requested maze is too large",
            ))
        } else {
            Ok(self.into())
        }
    }
}

impl From<Maze> for HttpResponse {
    fn from(mut source: Maze) -> Self {
        let maze = source
            .maze_type
            .create::<()>(source.dimensions)
            .initialize(initialize::Method::Branching, &mut source.seed);

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
            .set("viewBox", maze.viewbox().tuple())
            .add(container)
            .to_string();
        HttpResponse::Ok().content_type("image/svg+xml").body(data)
    }
}
