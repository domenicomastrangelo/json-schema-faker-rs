use crate::description::Property;

pub fn generate(p: &Property, rng: &mut impl rand::Rng) -> serde_json::Value {
    let mut min = 0;
    let mut max = 100;

    let opts = match p.options {
        Some(ref o) => o,
        None => return serde_json::json!(null),
    };

    if let Some(m) = opts.get("min") {
        min = m.as_i64().unwrap_or(min);
    }

    if let Some(m) = opts.get("max") {
        max = m.as_i64().unwrap_or(max);
    }

    serde_json::json!(rng.random_range(min..max))
}
