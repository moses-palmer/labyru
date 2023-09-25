use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

mod types;

#[derive(Deserialize)]
struct Query {
    seed: Option<types::Seed>,
    solve: Option<bool>,
}
#[get("/{maze_type}/{dimensions}/image.svg")]
async fn maze_svg(
    (path, query): (
        web::Path<(types::MazeType, types::Dimensions)>,
        web::Query<Query>,
    ),
) -> impl Responder {
    let (maze_type, dimensions) = path.into_inner();
    let Query { seed, solve } = query.into_inner();
    HttpResponse::from(types::Maze {
        maze_type,
        dimensions,
        seed: seed.unwrap_or_else(types::Seed::random),
        solve: solve.unwrap_or(false),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(maze_svg))
        .bind("0.0.0.0:8000")
        .unwrap()
        .run()
        .await
}
