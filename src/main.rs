#![feature(test)]
extern crate test;

use statrs::distribution::Weibull;
use statrs::distribution::ContinuousCDF;

#[derive(Debug)]
struct WeibullModel {
    shape: f64,
    scale: f64,
    reliability: Vec<f64>,
}

impl WeibullModel {
    fn new(shape: f64, scale: f64) -> Self {
        let reliability = reliability_over_time(shape, scale, 720);
        Self { shape, scale, reliability }
    }
}

fn reliability_over_time(shape: f64, scale: f64, steps: u32) -> Vec<f64> {
    let weibull = Weibull::new(shape, scale).unwrap();

    let mut cdf: Vec<f64> = Vec::new();
    for x in 0..steps {
        let val = weibull.cdf(x as f64);
        cdf.push(val);
    }

    let reliability: Vec<f64> = cdf.iter().map(|&x| 1.0 - x).collect();

    reliability
}

fn build_components(num_components: u32) -> Vec<WeibullModel> {
    let mut components: Vec<_> = Vec::new();
    for _ in 0..num_components {
        let component = WeibullModel::new(0.5, 200.0);
        components.push(component);
    }

    components
}

fn main() {
    let models = build_components(500_000);
    let last = models.last();
    println!("Final model: {:?}", last);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_cdf(b: &mut Bencher) {
        b.iter(|| build_components(100));
    }
}
