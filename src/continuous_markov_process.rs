use rand::Rng;
use rand_distr::{Exp, Distribution};

pub trait BirthAndDeathProbabilities {
    fn probability_tuple(&self, from_state: u64) -> (f64, f64) ;
}

pub struct MarkovQueueProbabilities {
    arrival_rate: f64,
    serive_rate: f64,
    num_units: u64,
}

impl MarkovQueueProbabilities {
    pub fn new(arrival_rate: f64, serive_rate: f64, units: u64) -> Self {
        assert!(arrival_rate > 0.);
        assert!(serive_rate > 0.);
        Self {
            arrival_rate,
            serive_rate,
            num_units: units,
        }
    }
}

impl BirthAndDeathProbabilities for MarkovQueueProbabilities {
    fn probability_tuple(&self, from_state: u64) -> (f64, f64) {
        let birth_rate = self.arrival_rate;
        let death_rate = self.serive_rate * from_state.min(self.num_units) as f64;
        (birth_rate, death_rate)
    }
}

pub trait ContinuousMarkovTransitions {
    fn next_transition(&self, from_state: u64, rng: &mut impl Rng) -> (u64, f64);
}

impl<T> ContinuousMarkovTransitions for T
where
    T: BirthAndDeathProbabilities,
{
    fn next_transition(&self, from_state: u64, rng: &mut impl Rng) -> (u64, f64) {
        let (birth_rate, death_rate) = self.probability_tuple(from_state);
        let total_rate = birth_rate + death_rate;
        let time_to_next_transition = Exp::new(total_rate).unwrap().sample(rng);
        let next_state = if rng.gen_bool(birth_rate / total_rate) {
            from_state + 1
        } else {
            from_state - 1
        };
        (next_state, time_to_next_transition)
    }
}

pub struct ContinuousMarkovProcess<M> where M: ContinuousMarkovTransitions {
    cur_state: u64,
    cur_time: f64,
    transitions: M,
}

impl<M> ContinuousMarkovProcess<M> where M: ContinuousMarkovTransitions {
    pub fn new(transitions: M, start_state: u64) -> Self {
        Self {
            cur_state: start_state,
            cur_time: 0.,
            transitions,
        }
    }

    pub fn time(&self) -> f64 {
        self.cur_time
    }

    pub fn state(&self) -> u64 {
        self.cur_state
    }

    pub fn step(&mut self, rng: &mut impl Rng) {
        let (next_state, time_delta) = self.transitions.next_transition(self.cur_state, rng);
        self.cur_state = next_state;
        self.cur_time += time_delta;
    }
}
