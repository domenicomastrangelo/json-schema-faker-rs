use chrono::{Datelike, Utc};

use crate::description::Property;

pub fn generate(p: &Property, rng: &mut impl rand::Rng) -> serde_json::Value {
    let min_year = p
        .options
        .as_ref()
        .and_then(|o| o.get("min").and_then(|m| m.as_i64()))
        .unwrap_or(1900);

    let max_year = p
        .options
        .as_ref()
        .and_then(|opts| opts.get("max").and_then(|m| m.as_i64()))
        .unwrap_or(i64::from(Utc::now().year()));

    let year = rng.random_range(min_year..=max_year);
    let month = rng.random_range(1..=12);
    let day = rng.random_range(1..=28);

    serde_json::json!(format!("{:04}-{:02}-{:02}", year, month, day))
}
