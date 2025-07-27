![Build](https://github.com/MathieuGr02/json_variables/workflows/Build/badge.svg)
![Rust version](https://img.shields.io/badge/rust-1.88+-blue.svg)

# JSON variables
Adds a variable system for JSON format through string parsing and pattern in rust.

## How to use

Given a JSON string with variables defined through a pattern (here `${<variable>}`), one can define the value of the variables and replace them in the JSON string.
```rust
use serde_json::json;

fn main() {
    let data = r#"
        [
            {
                "name": "Franklin",
                "country": "${NL}"
            },
            {
                "name": "John",
                "country": "${NL}"
            }
        ]"#;
    let variables = json!(
        {
            "NL": {
                "long": "Netherlands",
                "short": "NL"
            }
        }
    );
    let variables = json_variables::from_json(variables).expect("Unable to parse json to variables");
    let result = variables.replace(data); 
}
```
This would be the same as creating the JSON string

```json
[
  {
    "name": "Franklin",
    "country": {
      "long": "Netherlands",
      "short": "NL"
    }
  },
  {
    "name": "John",
    "country": {
      "long": "Netherlands",
      "short": "NL"
    }
  }
]
```

## Usage & Features

One way to use this variable system is by adding a field to ones JSON, defining the variables and a field with ones struct which will be deserialized by `serde_json`.

```rust
pub struct Country {
    short: String,
    long: String,
}

pub struct Person {
    name: String,
    country: Country,
}

fn main() {
    let data = r#"
        {
            "variables": {
                "NL": { "short": "NL", "long": "Netherlands" },
            }
            "config": {
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
            }
        }"#;
}

```

### Derive

A derive trait is provided which can be attached to structs where the variables can then be parsed through an intermediate struct.

```rust
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
```



The derive trait can be customized through the following attributes which can be set by adding `#[variables( ... )]` to the corresponding struct.

- `wrapper_name`: Change the wrapping struct name in case of name collision.
- `variables_name`: Change the variables struct name in case of name collision.
- `data_field_name`: Change the field name of the data in the JSON file. Works through `#[serde(alias = "name")]`.
- `variables_field_name`: Change the field name of the variables in the JSON file.
- `flatten_data_field`: Add `#[serde(flatten)]` to the struct for the variables struct