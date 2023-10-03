use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};
use maze::initialize::{Method, LFSR};
use maze::{Maze, Shape};

pub fn initialize(c: &mut Criterion) {
    for &method in [Method::Braid, Method::Branching, Method::Winding].iter() {
        let mut group = c.benchmark_group(format!("initialize {}", method));
        for shape in [Shape::Tri, Shape::Quad, Shape::Hex].iter() {
            group.bench_with_input(
                BenchmarkId::from_parameter(shape),
                shape,
                |b, &shape| {
                    b.iter(|| {
                        Maze::<()>::new(black_box(shape), 100, 100)
                            .initialize(method, &mut LFSR::new(65));
                    });
                },
            );
        }
        group.finish();
    }
}

criterion_group!(benches, initialize);
criterion_main!(benches);
