use std::{cmp::Reverse, collections::BinaryHeap};

use ordered_float::OrderedFloat;
use rand::Rng;
use rand_distr::{Distribution, Exp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct RoF64(Reverse<OrderedFloat<f64>>);

impl From<f64> for RoF64 {
    fn from(value: f64) -> Self {
        RoF64(Reverse(value.into()))
    }
}

impl Into<f64> for RoF64 {
    fn into(self) -> f64 {
        self.0 .0 .0
    }
}

pub trait QueueSystem {
    fn time(&self) -> f64;

    fn queue_length(&self) -> u64;

    fn step(&mut self, rng: &mut impl Rng);

    fn step_t(&mut self, delta_t: f64, rng: &mut impl Rng);

    fn add_arrival(&mut self, rng: &mut impl Rng);
}

pub struct GeneralQueueSystem<A, S>
where
    A: Distribution<f64>,
    S: Distribution<f64>,
{
    num_units: u64,
    arrival_distribution: A,
    service_distribution: S,
    length: u64,
    time: f64,
    next_arrival_time: f64,
    queue: BinaryHeap<RoF64>,
}

impl<A, S> GeneralQueueSystem<A, S>
where
    A: Distribution<f64>,
    S: Distribution<f64>,
{
    pub fn new(
        num_units: Option<u64>,
        arrival_distribution: A,
        service_distribution: S,
        start_length: u64,
        rng: &mut impl Rng,
    ) -> Self {
        if let Some(num_units) = num_units {
            assert_ne!(
                num_units, 0,
                "A queue system must contain at least one service unit. Use `None` for infinite."
            );
        }
        let next_arrival_time = arrival_distribution.sample(rng);
        let mut result = GeneralQueueSystem {
            num_units: num_units.unwrap_or(0),
            arrival_distribution,
            service_distribution,
            length: start_length,
            time: 0.,
            next_arrival_time,
            queue: BinaryHeap::new(),
        };
        result.fill_queue(rng);
        result
    }

    fn fill_queue(&mut self, rng: &mut impl Rng) {
        let max_queue_length = if self.num_units == 0 {
            self.length
        } else {
            self.length.min(self.num_units)
        } as usize;
        while self.queue.len() < max_queue_length {
            self.queue
                .push((self.time + self.service_distribution.sample(rng)).into())
        }
    }
}

impl<A, S> QueueSystem for GeneralQueueSystem<A, S>
where
    A: Distribution<f64>,
    S: Distribution<f64>,
{
    fn time(&self) -> f64 {
        self.time
    }

    fn queue_length(&self) -> u64 {
        self.length
    }

    fn step(&mut self, rng: &mut impl Rng) {
        if let Some(&next_service_time) = self
            .queue
            .peek()
            .filter(|&&next_service_time| self.next_arrival_time > next_service_time.into())
        {
            self.queue.pop();
            self.time = next_service_time.into();
            self.length -= 1;
        } else {
            self.time = self.next_arrival_time;
            self.next_arrival_time = self.time + self.arrival_distribution.sample(rng);
            self.length += 1;
            self.fill_queue(rng);
        }
    }

    fn step_t(&mut self, delta_t: f64, rng: &mut impl Rng) {
        assert!(
            delta_t >= 0.,
            "Cannot step backwards in time. Current time: {}, requested time: {}",
            self.time,
            delta_t
        );
        self.time += delta_t;
        while self.time > self.next_arrival_time {
            self.length += 1;
            self.fill_queue(rng);
            self.next_arrival_time += self.arrival_distribution.sample(rng);
        }
        while let Some(_) = self
            .queue
            .peek()
            .filter(|&&next_service_time| self.time > next_service_time.into())
        {
            self.length -= 1;
            self.queue.pop();
        }
    }

    fn add_arrival(&mut self, rng: &mut impl Rng) {
        self.length += 1;
        self.fill_queue(rng)
    }
}

struct MarkovServiceQueueSystem<A>
where
    A: Distribution<f64>,
{
    num_units: u64,
    arrival_distribution: A,
    service_rate: f64,
    length: u64,
    time: f64,
    next_arrival_time: f64,
    next_service_time: f64,
}

impl<A> MarkovServiceQueueSystem<A>
where
    A: Distribution<f64>,
{
    pub fn new(
        num_units: Option<u64>,
        arrival_distribution: A,
        service_rate: f64,
        start_length: u64,
        rng: &mut impl Rng,
    ) -> Self {
        if let Some(num_units) = num_units {
            assert_ne!(
                num_units, 0,
                "A queue system must contain at least one service unit. Use `None` for infinite."
            );
        }
        let next_arrival_time = arrival_distribution.sample(rng);
        let next_service_time = if start_length > 0 {
            Exp::new((start_length as f64) * service_rate)
                .unwrap()
                .sample(rng)
        } else {
            f64::INFINITY
        };
        MarkovServiceQueueSystem {
            num_units: num_units.unwrap_or(0),
            arrival_distribution,
            service_rate,
            length: start_length,
            time: 0.,
            next_arrival_time,
            next_service_time,
        }
    }
}

impl<A> QueueSystem for MarkovServiceQueueSystem<A>
where
    A: Distribution<f64>,
{
    fn time(&self) -> f64 {
        self.time
    }

    fn queue_length(&self) -> u64 {
        self.length
    }

    fn step(&mut self, rng: &mut impl Rng) {
        if self.next_service_time < self.next_arrival_time {
            self.time = self.next_service_time;
            self.length -= 1;
            if self.length > 0 {
                self.next_service_time +=
                    Exp::new(self.service_rate * self.length.min(self.num_units) as f64)
                        .unwrap()
                        .sample(rng);
            } else {
                self.next_service_time = f64::INFINITY;
            }
        } else {
            self.time = self.next_arrival_time;
            self.next_arrival_time += self.arrival_distribution.sample(rng);
            self.add_arrival(rng);
        }
    }

    fn step_t(&mut self, delta_t: f64, rng: &mut impl Rng) {
        assert!(
            delta_t >= 0.,
            "Cannot step backwards in time. Current time: {}, requested time: {}",
            self.time,
            delta_t
        );
        self.time += delta_t;
        while self.time > self.next_arrival_time.min(self.next_service_time) {
            if self.next_service_time < self.next_arrival_time {
                self.length -= 1;
                if self.length > 0 {
                    self.next_service_time +=
                        Exp::new(self.service_rate * self.length.min(self.num_units) as f64)
                            .unwrap()
                            .sample(rng);
                } else {
                    self.next_service_time = f64::INFINITY;
                }
            } else {
                self.next_arrival_time += self.arrival_distribution.sample(rng);
                self.add_arrival(rng);
            }
        }
    }

    fn add_arrival(&mut self, rng: &mut impl Rng) {
        if self.length == 0 {
            assert_eq!(self.next_service_time, f64::INFINITY);
            self.next_service_time = Exp::new(self.service_rate).unwrap().sample(rng);
            self.length += 1;
        } else {
            assert_ne!(self.next_service_time, f64::INFINITY);
            assert!(self.next_service_time > 0.);
            if self.length < self.num_units {
                self.next_service_time = self.time
                    + (self.next_service_time - self.time) * (self.length + 1) as f64
                        / self.length as f64;
            }
            self.length += 1;
        }
    }
}
