#[macro_use]
extern crate criterion;
extern crate stl_thumb;

use criterion::Criterion;
use stl_thumb::config::Config;

fn benchy() {
    let config = Config {
        stl_filename: "test_data/3DBenchy.obj".to_string(),
        img_filename: "benchy_obj.png".to_string(),
        width: 1024,
        height: 768,
        ..Default::default()
    };

    stl_thumb::render_to_file(&config).expect("Error in run function");
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("benchy_obj", |b| b.iter(|| benchy()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
