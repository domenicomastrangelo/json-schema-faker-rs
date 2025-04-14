use crate::description::Property;

use super::{
    string_address, string_birthday, string_city, string_country, string_email, string_job,
    string_name, string_phone, string_state, string_street, string_surname, string_zip,
};

pub fn generate(t: &str, p: &Property) -> serde_json::Value {
    let mut rng = rand::rng();

    match t {
        "name" => string_name::generate(&mut rng),
        "surname" => string_surname::generate(&mut rng),
        "email" => string_email::generate(&mut rng),
        "phone" => string_phone::generate(&mut rng),
        "birthday" => string_birthday::generate(p, &mut rng),
        "address" => string_address::generate(&mut rng),
        "city" => string_city::generate(&mut rng),
        "country" => string_country::generate(&mut rng),
        "state" => string_state::generate(&mut rng),
        "zip" => string_zip::generate(&mut rng),
        "street" => string_street::generate(&mut rng),
        "job" => string_job::generate(&mut rng),
        _ => serde_json::json!(null),
    }
}
