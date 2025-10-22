use crate::model::{Property, ExpenseKind};


pub struct MonthlyCashFlow {
pub income: f64,
pub expenses: f64,
pub net: f64,
}


pub fn cashflow_for_property(p: &Property, month_index: u32, vacancy: bool, mortgage_pmt: f64) -> MonthlyCashFlow {
// Inflate rent and HOA over time (very simple model)
let rent_growth = (1.0 + p.appreciation_rate_annual / 12.0).powi(month_index as i32);
let rent = if vacancy { 0.0 } else { p.rent_monthly * rent_growth };


let hoa = p.hoa_monthly * rent_growth;
let taxes = super::taxes::property_tax(p.purchase_price * rent_growth, p.tax_rate_annual) / 12.0;
let insurance = p.insurance_annual / 12.0;
let maintenance = (p.purchase_price * p.maintenance_rate_annual) / 12.0;


let income = rent;
let expenses = hoa + taxes + insurance + maintenance + mortgage_pmt;


MonthlyCashFlow { income, expenses, net: income - expenses }
}


pub fn expense_breakdown(p: &Property, month_index: u32, _mortgage_pmt: f64) -> Vec<(ExpenseKind, f64)> {
let rent_growth = (1.0 + p.appreciation_rate_annual / 12.0).powi(month_index as i32);
let hoa = p.hoa_monthly * rent_growth;
let taxes = super::taxes::property_tax(p.purchase_price * rent_growth, p.tax_rate_annual) / 12.0;
let insurance = p.insurance_annual / 12.0;
let maintenance = (p.purchase_price * p.maintenance_rate_annual) / 12.0;


// Mortgage is not an ExpenseKind; it is returned separately by cashflow_for_property via `mortgage_pmt`.
vec![
(ExpenseKind::HOA, hoa),
(ExpenseKind::Tax, taxes),
(ExpenseKind::Insurance, insurance),
(ExpenseKind::Maintenance, maintenance),
(ExpenseKind::Management, 0.0),
(ExpenseKind::CapEx, 0.0),
(ExpenseKind::Vacancy, 0.0),
]
}