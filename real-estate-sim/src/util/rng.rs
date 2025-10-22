use rand::{Rng, SeedableRng};


pub trait SmallRngExt {
fn seed_from_u64(seed: u64) -> Self;
fn normal_small(&mut self, sigma: f64) -> f64;
}


impl SmallRngExt for rand::rngs::SmallRng {
fn seed_from_u64(seed: u64) -> Self { rand::rngs::SmallRng::seed_from_u64(seed) }
fn normal_small(&mut self, sigma: f64) -> f64 {
if sigma <= 0.0 { return 0.0; }
// Marsaglia polar for standard normal
loop {
let u: f64 = self.gen_range(-1.0..1.0);
let v: f64 = self.gen_range(-1.0..1.0);
let s = u*u + v*v;
if s == 0.0 || s >= 1.0 { continue; }
let mul = (-2.0 * s.ln() / s).sqrt();
return u * mul * sigma;
}
}
}