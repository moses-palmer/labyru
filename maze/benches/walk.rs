use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};
use maze::initialize::{Method, LFSR};
use maze::{Maze, Shape};

pub fn walk(c: &mut Criterion) {
    for &method in [Method::Braid, Method::Branching, Method::Winding].iter() {
        let mut group = c.benchmark_group(format!("walk {}", method));
        for shape in [Shape::Tri, Shape::Quad, Shape::Hex].iter() {
            let maze = Maze::<()>::new(black_box(*shape), 100, 100)
                .initialize(method, &mut LFSR::new(65));
            let start = (0isize, 0isize).into();
            let end =
                ((maze.width() - 1) as isize, (maze.height() - 1) as isize)
                    .into();
            group.bench_with_input(
                BenchmarkId::from_parameter(shape),
                shape,
                |b, _| {
                    b.iter(|| {
                        maze.walk(start, end);
                    });
                },
            );
        }
        group.finish();
    }
}

criterion_group!(benches, walk);
criterion_main!(benches);
