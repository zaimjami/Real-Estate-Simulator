use serde::{Deserialize, Serialize};
use super::Property;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
pub name: String,
pub properties: Vec<Property>,
}


impl Portfolio {
pub fn total_starting_equity(&self) -> f64 { self.properties.iter().map(|p| p.equity_start()).sum() }
}