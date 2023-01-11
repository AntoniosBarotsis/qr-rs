use criterion::{criterion_group, criterion_main, Criterion};
use qr_rs_lib::QrCodeBuilder;
use rand::{distributions::Alphanumeric, Rng};

const LOGO: &[u8] = include_bytes!("../../assets/logo.png");

pub fn criterion_benchmark(c: &mut Criterion) {
  let s = generate_random_input();

  let _r = c.bench_function("Generate QR Code", |b| {
    b.iter(|| {
      let link = &s
        .get(rand::thread_rng().gen_range(0..100))
        .expect("Index in range")
        .clone();

      let _qr_code = QrCodeBuilder::new(link, LOGO)
        .build()
        .expect("Should not crash.");
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
