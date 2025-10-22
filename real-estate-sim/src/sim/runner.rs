use crate::engine::{AmortizationIter, cashflow_for_property};
use crate::model::{Portfolio};
use crate::sim::{result::{PropertyResult, PropertyTracePoint, SimulationOutcome}, config::SimulationConfig};
use crate::util::rng::SmallRngExt;
use rand::Rng;


pub fn run_scenario(cfg: &SimulationConfig, seed: u64) -> anyhow::Result<SimulationOutcome> {
let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
let months = cfg.months;


let mut total_cashflow = 0.0;
let mut outcomes: Vec<PropertyResult> = Vec::new();


for prop in &cfg.portfolio.properties {
let mut amort = AmortizationIter::new(&prop.mortgage);
let monthly_pmt = amort.monthly_payment();


let mut equity = prop.equity_start();
let mut value = prop.purchase_price;
let mut series = Vec::with_capacity(months as usize);


for m in 0..months {
// Stochastic vacancy
let vacancy = rng.gen_bool(cfg.market.vacancy_probability_monthly.clamp(0.0, 1.0));


// Cashflow
let cf = cashflow_for_property(prop, m, vacancy, monthly_pmt);
total_cashflow += cf.net;


// Appreciation (simple monthly drift)
let monthly_app = prop.appreciation_rate_annual / 12.0;
value *= 1.0 + monthly_app + rng.normal_small(cfg.market.rate_volatility / 12.0);


// Amortization principal paydown
if let Some(step) = amort.next() {
equity += step.principal;
}


series.push(PropertyTracePoint { month: m, equity, value_estimate: value, net_cashflow: cf.net });
}


outcomes.push(PropertyResult { id: prop.id.clone(), series });
}


let portfolio_equity_final: f64 = outcomes.iter().map(|r| r.series.last().map(|tp| tp.equity).unwrap_or(0.0)).sum();


Ok(SimulationOutcome { seed, portfolio_equity_final, total_cashflow, properties: outcomes })
}