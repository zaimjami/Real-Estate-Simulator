use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mortgage {
pub loan_type: super::LoanType,
pub principal: f64,
pub annual_rate: f64, // nominal APR
pub term_months: u32,
}


impl Mortgage {
pub fn monthly_rate(&self) -> f64 { self.annual_rate / 12.0 }
}