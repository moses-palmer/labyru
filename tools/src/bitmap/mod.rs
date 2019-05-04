use image;

use maze;

/// Converts an image to a matrix by calling an update function with a pixel
/// and its corresponding matrix position.
///
/// # Arguments
/// *  `image` - The image to convert.
/// *  `maze` - A template maze. This is used to determine which matrix
///    position a pixel corresponds to, and to determine the dimensions of the
///    matrix.
/// *  `update` - The update function.
pub fn image_to_matrix<U, T>(
    image: &image::RgbImage,
    maze: &maze::Maze,
    update: U,
) -> maze::matrix::Matrix<T>
where
    U: Fn(&mut maze::matrix::Matrix<T>, maze::matrix::Pos, &image::Rgb<u8>),
    T: Copy + Default,
{
    let (left, top, width, height) = maze.viewbox();
    let (cols, rows) = image.dimensions();
    image.enumerate_pixels().fold(
        maze::matrix::Matrix::<T>::new(maze.width(), maze.height()),
        |mut matrix, (x, y, pixel)| {
            let physical_pos = maze::physical::Pos {
                x: left + width * (x as f32 / cols as f32),
                y: top + height * (y as f32 / rows as f32),
            };
            let pos = maze.room_at(physical_pos);
            update(&mut matrix, pos, pixel);
            matrix
        },
    )
}
