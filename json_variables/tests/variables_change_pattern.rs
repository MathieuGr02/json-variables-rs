#[cfg(test)]
pub mod pattern {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use json_variables::JsonVariables;

    #[test]
    pub fn pattern() {
        #[derive(Debug, Deserialize, Serialize, JsonVariables)]
        struct PersonData {
            name: String,
            age: u8
        }

        let json = json!(
        {
            "variables":
            {
                "NAME": "John",
                "pattern": "\\?\\[([a-zA-Z0-9_.]+)\\]",
            },
            "person_data": { "name": "?[NAME]", "age": 23 }
        });

        let person_data_variables: PersonDataVariables = serde_json::from_value(json).expect("Unable to parse json to variable");
        let x = person_data_variables.parse().expect("Unable to set variables");
        assert_eq!(x.name, "John");
        assert_eq!(x.age, 23);
    }
}