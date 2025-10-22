use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyTracePoint {
pub month: u32,
pub equity: f64,
pub value_estimate: f64,
pub net_cashflow: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyResult { pub id: String, pub series: Vec<PropertyTracePoint> }


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationOutcome {
pub seed: u64,
pub portfolio_equity_final: f64,
pub total_cashflow: f64,
pub properties: Vec<PropertyResult>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateSummary {
pub runs: usize,
pub mean_equity: f64,
pub p5_equity: f64,
pub p95_equity: f64,
pub mean_cashflow: f64,
}


pub fn aggregate(runs: &[SimulationOutcome]) -> AggregateSummary {
let mut eq: Vec<f64> = runs.iter().map(|r| r.portfolio_equity_final).collect();
let mut cf: Vec<f64> = runs.iter().map(|r| r.total_cashflow).collect();
eq.sort_by(|a,b| a.partial_cmp(b).unwrap());
cf.sort_by(|a,b| a.partial_cmp(b).unwrap());
let n = eq.len().max(1);
let mean_equity = eq.iter().sum::<f64>() / n as f64;
let mean_cashflow = cf.iter().sum::<f64>() / n as f64;
let p5_equity = percentile(&eq, 5.0);
let p95_equity = percentile(&eq, 95.0);
AggregateSummary { runs: n, mean_equity, p5_equity, p95_equity, mean_cashflow }
}


fn percentile(sorted: &[f64], p: f64) -> f64 {
if sorted.is_empty() { return 0.0; }
let rank = (p / 100.0) * (sorted.len() - 1) as f64;
let lo = rank.floor() as usize;
let hi = rank.ceil() as usize;
if lo == hi { sorted[lo] } else { sorted[lo] + (sorted[hi] - sorted[lo]) * (rank - lo as f64) }
}