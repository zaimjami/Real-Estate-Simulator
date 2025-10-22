pub fn compound_monthly(balance: f64, monthly_rate: f64, months: u32) -> f64 {
    if monthly_rate == 0.0 { balance }
    else { balance * (1.0 + monthly_rate).powi(months as i32) }
    }