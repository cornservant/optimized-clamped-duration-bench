use criterion::{criterion_group, criterion_main, Criterion};
use optimized_clamped_duration_bench::*;

fn generate_input() -> Vec<(f64, f64, f64, f64, f64)> {
    use rand_distr::Distribution;
    use rand_distr::Exp;
    let mut rng = rand::rng();
    let dist_r = Exp::new(1.0 / 1.0).unwrap();
    let dist_s = Exp::new(1.0 / 1.0).unwrap();
    let dist_e = Exp::new(1.0 / 0.0001).unwrap();
    let dist_v = Exp::new(1.0 / 2.0).unwrap();
    let dist_t = Exp::new(1.0 / 10.0).unwrap();
    (0..500)
        .map(|_| {
            (
                dist_r.sample(&mut rng),
                dist_s.sample(&mut rng),
                dist_e.sample(&mut rng),
                dist_v.sample(&mut rng),
                dist_t.sample(&mut rng),
            )
        })
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = generate_input();
    let from = 0.0;

    let mut group = c.benchmark_group("day01::solve1");
    group.bench_with_input("baseline", &input, |b, input| {
        b.iter(|| {
            input
                .iter()
                .copied()
                .for_each(|(ratio, stiffness, eps, initial_velocity, to)| {
                    let params = baseline::SpringParams::new(ratio, stiffness, eps);
                    let spring = baseline::Spring {
                        from,
                        to,
                        initial_velocity,
                        params,
                    };
                    let _ = spring.clamped_duration();
                })
        })
    });
    group.bench_with_input("commit1", &input, |b, input| {
        b.iter(|| {
            input
                .iter()
                .copied()
                .for_each(|(ratio, stiffness, eps, initial_velocity, to)| {
                    let params = commit1::SpringParams::new(ratio, stiffness, eps);
                    let spring = commit1::Spring {
                        from,
                        to,
                        initial_velocity,
                        params,
                    };
                    let _ = spring.clamped_duration();
                })
        })
    });
    group.bench_with_input("commit2", &input, |b, input| {
        b.iter(|| {
            input
                .iter()
                .copied()
                .for_each(|(ratio, stiffness, eps, initial_velocity, to)| {
                    let params = commit2::SpringParams::new(ratio, stiffness, eps);
                    let spring = commit2::Spring {
                        from,
                        to,
                        initial_velocity,
                        params,
                    };
                    let _ = spring.clamped_duration();
                })
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
