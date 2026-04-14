use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use maze::initialize::{LFSR, Method};
use maze::{Maze, Shape};

pub fn walk(c: &mut Criterion) {
    for &method in [Method::Braid, Method::Branching, Method::Winding].iter() {
        let mut group = c.benchmark_group(format!("walk {}", method));
        for shape in [Shape::Tri, Shape::Quad, Shape::Hex].iter() {
            let maze =
                Maze::<()>::new(black_box(*shape), 100, 100).initialize(method, &mut LFSR::new(65));
            let start = (0, 0).into();
            let end = ((maze.width() - 1) as i32, (maze.height() - 1) as i32).into();
            group.bench_with_input(BenchmarkId::from_parameter(shape), shape, |b, _| {
                b.iter(|| {
                    maze.walk(start, end);
                });
            });
        }
        group.finish();
    }
}

criterion_group!(benches, walk);
criterion_main!(benches);
