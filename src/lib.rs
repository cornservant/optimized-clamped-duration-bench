pub mod baseline;
pub mod commit1;
pub mod commit2;

#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub damping_ratio: f64,
    pub stiffness: f64,
    pub epsilon: f64,
    pub from: f64,
    pub to: f64,
    pub initial_velocity: f64,
}

pub fn generate_input() -> Vec<Params> {
    use rand_distr::Distribution;
    use rand_distr::Exp;
    let mut rng = rand::rng();
    let dist_d = Exp::new(1.0 / 2.0).unwrap();
    let dist_s = Exp::new(1.0 / 2.0).unwrap();
    let dist_e = Exp::new(1.0 / 0.0001).unwrap();
    let dist_v = Exp::new(1.0 / 2.0).unwrap();
    let dist_t = Exp::new(1.0 / 10.0).unwrap();
    (0..500)
        .map(|_| Params {
            damping_ratio: dist_d.sample(&mut rng),
            stiffness: dist_s.sample(&mut rng),
            epsilon: dist_e.sample(&mut rng),
            initial_velocity: dist_v.sample(&mut rng),
            from: 0.0,
            to: dist_t.sample(&mut rng),
        })
        .collect()
}
