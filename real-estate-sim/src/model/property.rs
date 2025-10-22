use serde::{Deserialize, Serialize};
use super::{Mortgage, PropertyType};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
pub id: String,
pub kind: PropertyType,
pub purchase_price: f64,
pub down_payment: f64,
pub closing_costs: f64,
pub rent_monthly: f64,
pub hoa_monthly: f64,
pub tax_rate_annual: f64, // % of assessed value
pub insurance_annual: f64,
pub maintenance_rate_annual: f64, // % of value
pub appreciation_rate_annual: f64,
pub mortgage: Mortgage,
}


impl Property {
pub fn equity_start(&self) -> f64 { self.down_payment + self.closing_costs }
pub fn basis(&self) -> f64 { self.purchase_price + self.closing_costs }
}