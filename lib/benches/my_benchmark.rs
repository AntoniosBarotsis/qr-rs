use criterion::{criterion_group, criterion_main, Criterion};
use qr_rs_lib::{generate_qr_code, DEFAULT_SIZE};
use rand::{distributions::Alphanumeric, Rng};

pub fn criterion_benchmark(c: &mut Criterion) {
  let s = generate_random_input();

  let _r = c.bench_function("Generate QR Code", |b| {
    b.iter(|| {
      generate_qr_code(
        &s.get(rand::thread_rng().gen_range(0..100))
          .expect("Index in range")
          .clone(),
        DEFAULT_SIZE,
        None,
      )
    });
  });
}

fn generate_random_input() -> Vec<String> {
  (0..100)
    .map(|_| {
      (0..150)
        .map(|_| rand::thread_rng().sample(Alphanumeric) as char)
        .collect::<String>()
    })
    .collect::<Vec<_>>()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
