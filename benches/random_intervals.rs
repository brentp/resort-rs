use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use resort::intersection::{Intersection, IntersectionIterator};
use resort::position::{Positioned, PositionedIterator};
use smartstring::alias::String;
use std::collections::HashMap;

#[derive(Debug)]
struct Interval {
    chrom: String,
    start: u64,
    stop: u64,
}

impl Positioned for Interval {
    fn start(&self) -> u64 {
        self.start
    }
    fn stop(&self) -> u64 {
        self.stop
    }
    fn chrom(&self) -> &str {
        &self.chrom
    }
}
struct Intervals {
    i: usize,
    name: String,
    n_intervals: usize,
    curr_max: f64,
    rng: rand::rngs::ThreadRng,
    interval_len: u64,
}

impl Intervals {
    fn new(name: String, n_intervals: usize, interval_len: u64) -> Self {
        Intervals {
            i: 0,
            name: name,
            n_intervals,
            curr_max: 1.0,
            rng: rand::thread_rng(),
            interval_len: interval_len,
        }
    }
}

impl PositionedIterator for Intervals {
    type Item = Interval;

    fn name(&self) -> String {
        String::from(format!("{}:{}", self.name, self.i))
    }

    fn next(&mut self) -> Option<Interval> {
        if self.i < self.n_intervals {
            self.i += 1;
            let r: f64 = self.rng.gen();
            self.curr_max *= r.powf(self.i as f64);
            let start = ((1.0 - self.curr_max) * (MAX_POSITION as f64)) as u64;
            Some(Interval {
                chrom: String::from("chr1"),
                start: start,
                stop: start + self.interval_len,
            })
        } else {
            None
        }
    }
}

const MAX_POSITION: u64 = 100_000;

pub fn intersection_benchmark(c: &mut Criterion) {
    let chrom_order = HashMap::from([(String::from("chr1"), 0), (String::from("chr2"), 1)]);

    c.bench_function("simple intersection", |b| {
        b.iter(|| {
            let a_ivs = Intervals::new(String::from("a"), 1000, 1000);
            let b_ivs = Intervals::new(String::from("b"), 10_000, 100);
            let iter = IntersectionIterator::new(a_ivs, vec![b_ivs], &chrom_order);

            iter.for_each(|intersection| {
                black_box(intersection.overlapping);
            });
        });
    });
}

criterion_group!(benches, intersection_benchmark);
criterion_main!(benches);
