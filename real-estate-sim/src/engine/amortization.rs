use crate::model::Mortgage;


pub struct AmortizationIter {
remaining: f64,
r: f64,
n: u32,
payment: f64,
month: u32,
}


impl AmortizationIter {
pub fn new(m: &Mortgage) -> Self {
let r = m.monthly_rate();
let n = m.term_months;
let p = m.principal;
let payment = if r == 0.0 { p / n as f64 } else { p * (r * (1.0 + r).powi(n as i32)) / ((1.0 + r).powi(n as i32) - 1.0) };
Self { remaining: p, r, n, payment, month: 0 }
}


pub fn monthly_payment(&self) -> f64 { self.payment }
}


pub struct AmortizationStep { pub interest: f64, pub principal: f64, pub remaining: f64 }


impl Iterator for AmortizationIter {
type Item = AmortizationStep;
fn next(&mut self) -> Option<Self::Item> {
if self.month >= self.n { return None; }
let interest = self.remaining * self.r;
let principal = (self.payment - interest).max(0.0);
self.remaining = (self.remaining - principal).max(0.0);
self.month += 1;
Some(AmortizationStep { interest, principal, remaining: self.remaining })
}
}