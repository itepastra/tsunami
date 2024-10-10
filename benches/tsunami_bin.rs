use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::random;
use tsunami::{binary, CanvasSize, Proto};

pub fn benchmark_send(c: &mut Criterion) {
    c.bench_function("send bin frame", |b| {
        async fn fun_name(
            (mut proto, canvas, size, mut writer): (binary::Protocol, u8, CanvasSize, Vec<u8>),
        ) {
            let color = tsunami::Color::RGB24(random(), random(), random());
            proto
                .send_frame(&mut writer, canvas, color, &size)
                .await
                .expect("should not fail");
        }
        b.iter_batched(
            || {
                let proto = binary::Protocol { count: 0 };
                let size = criterion::black_box(CanvasSize { x: 800, y: 600 });
                let canvas = random();
                return (proto, canvas, size, Vec::new());
            },
            fun_name,
            BatchSize::SmallInput,
        )
    });
}

pub fn benchmark_receive(c: &mut Criterion) {
    c.bench_function("receive bin frame", |b| {
        async fn fun_name(
            (mut proto, canvas, size, mut writer): (binary::Protocol, u8, CanvasSize, Vec<u8>),
        ) {
            proto
                .get_frame(&mut writer, canvas, &size)
                .await
                .expect("should not fail");
        }
        b.iter_batched(
            || {
                let proto = binary::Protocol { count: 0 };
                let size = criterion::black_box(CanvasSize { x: 800, y: 600 });
                let canvas = random();
                return (proto, canvas, size, Vec::new());
            },
            fun_name,
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, benchmark_send, benchmark_receive);
criterion_main!(benches);
