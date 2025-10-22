use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PropertyType { SingleFamily, MultiFamily, Condo, Townhome }


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoanType { Fixed30, Fixed15, ARM5x1 }


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExpenseKind { Tax, Insurance, HOA, Maintenance, CapEx, Vacancy, Management }


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EventKind {
Purchase,
RentCollected,
Expense(ExpenseKind),
Appreciation,
MortgagePayment,
}