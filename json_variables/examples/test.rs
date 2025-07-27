use serde::{Deserialize, Serialize};
use derive_json_variables::JsonVariables;

#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    short: String,
    long: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    name: String,
    country: Country,
}

#[derive(Debug, Deserialize, Serialize, JsonVariables)]
#[variables(flatten_data_field)]
pub struct Config {
    group: Vec<Person>
}

fn main() {
    let json = r#"
        {
            "variables": {
                "NL": { "short": "NL", "long": "Netherlands" }
            },
            "group":
            [
                {
                    "name": "Franklin",
                    "country": "${NL}"
                },
                {
                    "name": "John",
                    "country": "${NL}"
                }
            ]
        }"#;

    let person_variables: ConfigVariables = serde_json::from_str(json).expect("Unable to parse json to variable");
    let config = person_variables.parse().expect("Unable to set variables");

    for person in config.group {
        println!("{} is from the {} ({})", person.name, person.country.long, person.country.short);
    }
}