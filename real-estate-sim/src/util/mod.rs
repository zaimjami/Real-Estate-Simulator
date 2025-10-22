pub mod rng;
pub mod time;


use std::fs::File;
use std::io::BufReader;
use serde::de::DeserializeOwned;


pub fn load_json<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
let f = File::open(path)?;
let br = BufReader::new(f);
let t = serde_json::from_reader(br)?;
Ok(t)
}