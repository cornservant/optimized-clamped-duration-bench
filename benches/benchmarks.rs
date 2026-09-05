use criterion::{criterion_group, criterion_main, Criterion};
use optimized_clamped_duration_bench::*;

fn criterion_benchmark(c: &mut Criterion) {
    let input = generate_input();

    let mut group = c.benchmark_group("Spring::clamped_duration()");
    group.bench_with_input("baseline", &input, |b, input| {
        b.iter(|| {
            input.iter().copied().for_each(
                |Params {
                     damping_ratio,
                     stiffness,
                     epsilon,
                     initial_velocity,
                     from,
                     to,
                 }| {
                    let spring = baseline::Spring {
                        from,
                        to,
                        initial_velocity,
                        params: baseline::SpringParams::new(damping_ratio, stiffness, epsilon),
                    };
                    let _ = spring.clamped_duration();
                },
            )
        })
    });
    group.bench_with_input("commit1", &input, |b, input| {
        b.iter(|| {
            input.iter().copied().for_each(
                |Params {
                     damping_ratio,
                     stiffness,
                     epsilon,
                     initial_velocity,
                     from,
                     to,
                 }| {
                    let spring = commit1::Spring {
                        from,
                        to,
                        initial_velocity,
                        params: commit1::SpringParams::new(damping_ratio, stiffness, epsilon),
                    };
                    let _ = spring.clamped_duration();
                },
            )
        })
    });
    group.bench_with_input("commit2", &input, |b, input| {
        b.iter(|| {
            input.iter().copied().for_each(
                |Params {
                     damping_ratio,
                     stiffness,
                     epsilon,
                     initial_velocity,
                     from,
                     to,
                 }| {
                    let spring = commit2::Spring {
                        from,
                        to,
                        initial_velocity,
                        params: commit2::SpringParams::new(damping_ratio, stiffness, epsilon),
                    };
                    let _ = spring.clamped_duration();
                },
            )
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
