#![feature(test)]
extern crate test;

use statrs::distribution::Weibull;
use statrs::distribution::ContinuousCDF;

struct WeibullModel {
    shape: f64,
    scale: f64,
    cdf: Vec<f64>,
}

impl WeibullModel {
    fn new(shape: f64, scale: f64) -> Self {
        let cdf = cdf_over_time(shape, scale, 1_000_000);
        Self { shape, scale, cdf }
    }
}

fn cdf_over_time(shape: f64, scale: f64, steps: u32) -> Vec<f64> {
    let weibull = Weibull::new(shape, scale).unwrap();

    let mut cdf: Vec<f64> = Vec::new();
    for x in 0..steps {
        let val = weibull.cdf(x as f64);
        cdf.push(val);
    }

    cdf
}

fn build_components(num_components: u32) -> Vec<WeibullModel> {
    let mut components: Vec<_> = Vec::new();
    for _ in 0..num_components {
        let component = WeibullModel::new(0.5, 1.0);
        components.push(component);
    }

    components
}

fn main() {
    let _models = build_components(10_000);
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
