use rand::Rng;
use rand_distr::{Distribution, Normal};

pub struct BrownianMotion {
    cur_value: f64,
    step_distr: Normal<f64>,
}

impl BrownianMotion {
    pub fn initialize() -> Self {
        Self {
            cur_value: 0.,
            step_distr: Normal::new(0., 1.).unwrap(),
        }
    }

    pub fn step(&mut self, step_size: f64, rng: &mut impl Rng) -> f64 {
        self.cur_value += self.step_distr.sample(rng) * step_size.sqrt();
        self.cur_value
    }

    pub fn cur_value(&self) -> f64 {
        self.cur_value
    }
}

pub struct GeometricBrownianMotion {
    base_motion: BrownianMotion,
    cur_value: f64,
    cur_t: f64,
    start_value: f64,
    drift: f64,
    std_dev: f64,
}

impl GeometricBrownianMotion {
    pub fn initialize(start_value: f64, alpha: f64, variance: f64) -> Self {
        Self {
            base_motion: BrownianMotion::initialize(),
            cur_value: start_value,
            cur_t: 0.,
            start_value,
            drift: alpha - 0.5 * variance,
            std_dev: variance.sqrt(),
        }
    }

    pub fn cur_value(&self) -> f64 {
        self.cur_value
    }

    pub fn cur_t(&self) -> f64 {
        self.cur_t
    }

    pub fn step(&mut self, step_size: f64, rng: &mut impl Rng) -> f64 {
        self.cur_t += step_size;
        self.cur_value = self.start_value
            * (self.drift * self.cur_t + self.std_dev * self.base_motion.step(step_size, rng))
                .exp();
        self.cur_value
    }
}
