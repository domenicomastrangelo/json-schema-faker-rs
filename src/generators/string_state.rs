const STATE_LIST: [&str; 114] = [
    "Alabama",
    "Alaska",
    "Alberta",
    "American Samoa",
    "Arizona",
    "Arkansas",
    "Armed Forces (AE)",
    "Armed Forces Americas",
    "Armed Forces Pacific",
    "British Columbia",
    "California",
    "Colorado",
    "Connecticut",
    "Delaware",
    "District Of Columbia",
    "Florida",
    "Georgia",
    "Guam",
    "Hawaii",
    "Idaho",
    "Illinois",
    "Indiana",
    "Iowa",
    "Kansas",
    "Kentucky",
    "Louisiana",
    "Manitoba",
    "Maryland",
    "Massachusetts",
    "Michigan",
    "Minnesota",
    "Mississippi",
    "Missouri",
    "Montana",
    "Nebraska",
    "New Brunswick",
    "Nevada",
    "Newfoundland",
    "New Hampshire",
    "New Jersey",
    "New Mexico",
    "New York",
    "North Carolina",
    "North Dakota",
    "Northwest Territories",
    "Nova Scotia",
    "Nunavut",
    "Ohio",
    "Oklahoma",
    "Ontario",
    "Oregon",
    "Pennsylvania",
    "Prince Edward Island",
    "Puerto Rico",
    "Quebec",
    "Rhode Island",
    "Saskatchewan",
    "South Carolina",
    "South Dakota",
    "Tennessee",
    "Texas",
    "U.S. Virgin Islands",
    "Utah",
    "Vermont",
    "Virginia",
    "Washington",
    "West Virginia",
    "Wisconsin",
    "Wyoming",
    "Yukon Territory",
    "Aguascalientes",
    "Baja California",
    "Baja California Sur",
    "Campeche",
    "Chiapas",
    "Chihuahua",
    "Coahuila",
    "Colima",
    "Durango",
    "Federal District",
    "Guanajuato",
    "Guerrero",
    "Hidalgo",
    "Jalisco",
    "Mexico State",
    "Michoacán",
    "Morelos",
    "Nayarit",
    "Nuevo León",
    "Oaxaca",
    "Puebla",
    "Querétaro",
    "Quintana Roo",
    "San Luis Potosí",
    "Sinaloa",
    "Sonora",
    "Tabasco",
    "Tamaulipas",
    "Tlaxcala",
    "Veracruz",
    "Yucatán",
    "Zacatecas",
    "Acre",
    "Alagoas",
    "Amapá",
    "Amazonas",
    "Bahia",
    "Ceará",
    "Distrito Federal",
    "Essen",
    "Goiás",
    "Maranhão",
    "Mato Grosso",
    "Mato Grosso do Sul",
];

pub fn generate(rng: &mut impl rand::Rng) -> serde_json::Value {
    let state = rng.gen_range(0..STATE_LIST.len());

    serde_json::json!(STATE_LIST[state])
}
