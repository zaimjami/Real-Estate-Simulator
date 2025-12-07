# Real Estate Market Simulator  (Rust)

A lightweight, event-driven real estate market simulator written in **Rust**, designed to explore how property values, investor behavior, and market conditions interact over time.  
This project focuses on correctness, clean architecture, and fast iteration — taking advantage of Rust’s guarantees around safety and performance.

Overview

The simulator models a simplified real-estate environment where:

- Properties appreciate or depreciate based on configurable parameters  
- Investors buy/sell properties based on strategy rules  
- Market events influence overall behavior  
- The system evolves through discrete “ticks” in a simulation loop  

The intention is to build a small but expressive engine that shows how **stateful systems**, **data modeling**, and **economic logic** can be represented cleanly in Rust.

Key Features

- **Event-driven simulation architecture**  
- **Configurable market parameters** (volatility, appreciation, behavior rules)  
- **Structured Rust modeling** of Property, Investor, and MarketState  
- **Deterministic, safe execution** using Rust’s ownership model  
- **Extensible engine** — add new event types, strategies, or market mechanics with minimal changes

Tech Stack

- **Rust**  
- Cargo  
- (Optional) Serde for data handling  
- (Optional) Rand for stochastic behavior

Running the Project

1. Clone the repository:

```bash
git clone https://github.com/zaimjami/Real-Estate-Simulator.git
cd Real-Estate-Simulator

Run the simulation:

cargo run


(Optional) Run tests:

cargo test
