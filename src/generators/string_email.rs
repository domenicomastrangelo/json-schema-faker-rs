use super::{string_name, string_surname};

pub fn generate(mut rng: &mut impl rand::Rng) -> serde_json::Value {
    let name = match string_name::generate(&mut rng) {
        serde_json::Value::String(name) => name,
        _ => return serde_json::json!(null),
    };
    let surname = match string_surname::generate(&mut rng) {
        serde_json::Value::String(surname) => surname,
        _ => return serde_json::json!(null),
    };

    let random_number: u8 = rng.random_range(50..99);

    let email = format!("{}.{}{}@example.com", name, surname, random_number);

    serde_json::json!(email.to_lowercase())
}
