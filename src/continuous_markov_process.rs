use ndarray::Axis;
use rand::Rng;
use rand_distr::{Exp, Distribution};

use crate::{Matrix, Vector};

pub trait BirthAndDeathProbabilities {
    fn probability_tuple(&self, from_state: u64) -> (f64, f64) ;
}

#[derive(Debug, Clone)]
pub struct MarkovQueueProbabilities {
    arrival_rate: f64,
    service_rate: f64,
    num_units: u64,
}

impl MarkovQueueProbabilities {
    pub fn new(arrival_rate: f64, service_rate: f64, num_units: u64) -> Self {
        assert!(arrival_rate > 0.);
        assert!(service_rate > 0.);
        Self {
            arrival_rate,
            service_rate,
            num_units,
        }
    }
}

impl BirthAndDeathProbabilities for MarkovQueueProbabilities {
    fn probability_tuple(&self, from_state: u64) -> (f64, f64) {
        let birth_rate = self.arrival_rate;
        let death_rate = self.service_rate * from_state.min(self.num_units) as f64;
        (birth_rate, death_rate)
    }
}

pub trait ContinuousMarkovTransitions {
    fn next_transition(&self, from_state: u64, rng: &mut impl Rng) -> Option<(u64, f64)>;
}

impl<T> ContinuousMarkovTransitions for T
where
    T: BirthAndDeathProbabilities,
{
    fn next_transition(&self, from_state: u64, rng: &mut impl Rng) -> Option<(u64, f64)> {
        let (birth_rate, death_rate) = self.probability_tuple(from_state);
        let total_rate = birth_rate + death_rate;
        let time_to_next_transition = Exp::new(total_rate).unwrap().sample(rng);
        let next_state = if rng.gen_bool(birth_rate / total_rate) {
            from_state + 1
        } else {
            from_state - 1
        };
        Some((next_state, time_to_next_transition))
    }
}

#[derive(Debug, Clone)]
pub struct MatrixTransitions {
    total_rates: Vector,
    cumulative_rows: Matrix,
}

impl MatrixTransitions {
    pub fn new(transitions: Matrix) -> Self {
        assert!(transitions.is_square(), "Transition matrix must be square");
        let total_rates = -transitions.diag().to_owned();
        let mut cumulative_rows = transitions;
        for (k, mut row) in cumulative_rows.axis_iter_mut(Axis(0)).enumerate() {

            let total_rate = total_rates[k];
            if total_rate == 0. {
                continue;
            }
            row[k] = 0.;
            row /= total_rate;
            let mut cumulative_rate = 0.;
            for rate in row.iter_mut() {
                cumulative_rate += *rate;
                *rate = cumulative_rate;
            }
        }
        Self { total_rates, cumulative_rows }
    }
}

impl ContinuousMarkovTransitions for MatrixTransitions {
    fn next_transition(&self, from_state: u64, rng: &mut impl Rng) -> Option<(u64, f64)> {
        let from_state = from_state as usize;
        assert!(from_state <= self.total_rates.dim(), "Invalid state. Maximum state is {}", self.total_rates.dim());
        if from_state == self.total_rates.dim() {
            None
        } else {
            let cumulative_rates = self.cumulative_rows.row(from_state);
            let total_rate = self.total_rates[from_state];
            let time_to_next_transition = Exp::new(total_rate).unwrap().sample(rng);

            let rng_value = rng.gen_range(0. ..1.);
            let next_state = cumulative_rates.iter().position(|&cumulative_rate| {cumulative_rate > rng_value}).unwrap_or(cumulative_rates.len()) as u64;
            Some((next_state, time_to_next_transition))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContinuousMarkovProcess<M> where M: ContinuousMarkovTransitions {
    cur_state: u64,
    absorbed: bool,
    cur_time: f64,
    transitions: M,
}

impl<M> ContinuousMarkovProcess<M> where M: ContinuousMarkovTransitions {
    pub fn new(transitions: M, start_state: u64) -> Self {
        Self {
            cur_state: start_state,
            cur_time: 0.,
            transitions,
            absorbed: false,
        }
    }

    pub fn time(&self) -> f64 {
        self.cur_time
    }

    pub fn state(&self) -> u64 {
        self.cur_state
    }

    pub fn is_absorbed(&self) -> bool {
        self.absorbed
    }

    pub fn step(&mut self, rng: &mut impl Rng) {
        if self.absorbed {
            return;
        }
        if let Some((next_state, time_delta)) = self.transitions.next_transition(self.cur_state, rng) {
            self.cur_state = next_state;
            self.cur_time += time_delta;
        } else {
            self.absorbed = true;
        }
    }
}
