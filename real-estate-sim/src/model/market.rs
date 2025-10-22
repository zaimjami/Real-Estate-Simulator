use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketParams {
pub inflation_annual: f64,
pub rent_growth_annual: f64,
pub rate_volatility: f64,
pub vacancy_probability_monthly: f64,
}