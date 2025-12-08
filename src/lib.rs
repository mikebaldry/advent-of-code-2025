#![allow(dead_code)]

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_6_bad;
mod day_7;
mod day_8;

use std::time::{Duration, Instant};

pub fn bench(n: u32, f: fn()) {
    let mut sum = Duration::from_secs(0);
    let mut min = None;
    let mut max = None;

    f();

    for _ in 0..n {
        let t = Instant::now();
        f();
        let d = t.elapsed();
        if min.is_none() || min.unwrap() > d {
            min = Some(d);
        }
        if max.is_none() || max.unwrap() < d {
            max = Some(d);
        }
        sum += t.elapsed();
    }

    let avg = sum / n;
    println!(
        "min={:?} max={:?} avg={:?}",
        min.unwrap(),
        max.unwrap(),
        avg
    );
}
