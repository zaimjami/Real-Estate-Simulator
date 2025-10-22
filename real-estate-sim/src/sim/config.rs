use crate::model::{LoanType, MarketParams, Mortgage, Portfolio, Property, PropertyType};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
pub runs: usize,
pub months: u32,
pub market: MarketParams,
pub portfolio: Portfolio,
}


impl Default for SimulationConfig {
fn default() -> Self {
Self {
runs: 128,
months: 360,
market: MarketParams {
inflation_annual: 0.02,
rent_growth_annual: 0.03,
rate_volatility: 0.01,
vacancy_probability_monthly: 0.05,
},
portfolio: Portfolio {
name: "Demo".into(),
properties: vec![Property {
id: "SFH-001".into(),
kind: PropertyType::SingleFamily,
purchase_price: 400_000.0,
down_payment: 80_000.0,
closing_costs: 6_000.0,
rent_monthly: 2400.0,
hoa_monthly: 0.0,
tax_rate_annual: 0.007,
insurance_annual: 1200.0,
maintenance_rate_annual: 0.01,
appreciation_rate_annual: 0.03,
mortgage: Mortgage { loan_type: LoanType::Fixed30, principal: 320_000.0, annual_rate: 0.065, term_months: 360 },
}],
},
}
}
}