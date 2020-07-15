use criterion::{black_box, criterion_group, criterion_main, Criterion};
use grid;

pub fn criterion_benchmark(c: &mut Criterion) {
  let grid_cols = 3;
  let grid_rows = 3;
  let image_width = 1920;
  let image_height = 1080;

  let grid_size_width = image_width / grid_cols;
  let grid_size_height = image_height / grid_rows;

  let mut group = c.benchmark_group("loop");
  group.bench_function("one_loop", |b| {
    b.iter(|| {
      black_box(grid::one_loop(
        &grid_cols,
        &grid_rows,
        &grid_size_width,
        &grid_size_height,
      ))
    })
  });
  group.bench_function("two_loop", |b| {
    b.iter(|| {
      black_box(grid::two_loop(
        &grid_cols,
        &grid_rows,
        &grid_size_width,
        &grid_size_height,
      ))
    })
  });
  group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
