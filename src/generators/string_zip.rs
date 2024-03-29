const ZIP_LIST: [&str; 114] = [
    "10001",
    "EC1A 1BB",
    "8002",
    "10116",
    "0402",
    "100-0005",
    "K1A 0B2",
    "GPO Box 9995",
    "4002",
    "00122",
    "01101",
    "2801",
    "370002",
    "SE1 9GG",
    "100-8995",
    "1002",
    "1012",
    "1005",
    "M2 1AA",
    "E20 1EE",
    "01001",
    "1003",
    "1018",
    "10179",
    "100-8112",
    "1042",
    "2002",
    "100-6302",
    "1004",
    "00188",
    "1006",
    "W2A 1AA",
    "110002",
    "100-0006",
    "2003",
    "1005",
    "1043",
    "2004",
    "1007",
    "V6B 6G2",
    "AB11 1AB",
    "8003",
    "12436",
    "00101",
    "1008",
    "1014",
    "1009",
    "1044",
    "1015",
    "1016",
    "1017",
    "1018",
    "1019",
    "1020",
    "1021",
    "1022",
    "1023",
    "1024",
    "1025",
    "1026",
    "1027",
    "1028",
    "1029",
    "1030",
    "1031",
    "1032",
    "1033",
    "1034",
    "1035",
    "1036",
    "1037",
    "1038",
    "1039",
    "1040",
    "1041",
    "1042",
    "1043",
    "1044",
    "1045",
    "1046",
    "1047",
    "1048",
    "1049",
    "1050",
    "1051",
    "1052",
    "1053",
    "1054",
    "1055",
    "1056",
    "1057",
    "BX1 1LT",
    "CA1 1ER",
    "DA1 1RT",
    "EX1 1NT",
    "FY1 1VE",
    "GY1 1ND",
    "HX1 1ST",
    "IM1 1MM",
    "JE1 1EP",
    "KY1 1NG",
    "LX1 1UX",
    "ME1 1TE",
    "NE1 1EE",
    "OX1 1XF",
    "PA1 1SS",
    "QR1 1TY",
    "RE1 1AL",
    "SY1 1EM",
    "TW1 1CH",
    "UX1 1BE",
    "WY1 1LD",
    "YO1 1PL",
    "ZE1 1RO",
];

pub fn generate(rng: &mut impl rand::Rng) -> serde_json::Value {
    let index = rng.gen_range(0..ZIP_LIST.len());

    serde_json::json!(ZIP_LIST[index])
}
