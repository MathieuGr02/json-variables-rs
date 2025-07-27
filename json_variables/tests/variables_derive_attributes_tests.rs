#[cfg(test)]
mod variables_attributes {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use derive_json_variables::JsonVariables;

    #[test]
    pub fn data_field_name_default() {
        #[derive(Debug, Deserialize, Serialize, JsonVariables)]
        struct PersonData {
            name: String,
            age: u8
        }

        let json = json!(
        {
            "variables": { "NAME": "John" },
            "person_data": { "name": "${NAME}", "age": 23 }
        });

        let person_data_variables: PersonDataVariables = serde_json::from_value(json).expect("Unable to parse json to variable");
        let x = person_data_variables.parse().expect("Unable to set variables");
        assert_eq!(x.name, "John");
        assert_eq!(x.age, 23);
    }

    #[test]
    pub fn data_field_name_camel_case() {
        #[derive(Debug, Deserialize, Serialize, JsonVariables)]
        #[variables(data_field_name = "personData")]
        struct PersonData {
            name: String,
            age: u8
        }

        let json = json!(
        {
            "variables": { "NAME": "John" },
            "personData": { "name": "${NAME}", "age": 23 }
        });

        let person_data_variables: PersonDataVariables = serde_json::from_value(json).expect("Unable to parse json to variable");
        let x = person_data_variables.parse().expect("Unable to set variables");
        assert_eq!(x.name, "John");
        assert_eq!(x.age, 23);
    }

    /// Will simply not compile if the struct does not exist
    #[test]
    pub fn wrapper_name() {
        #[derive(Debug, Deserialize, Serialize, JsonVariables)]
        #[variables(wrapper_name = PersonDataNewWrapper)]
        struct PersonData {
            name: String,
            age: u8
        }

        let _ = PersonDataNewWrapper { person_data: PersonData { name: "".to_string(), age: 23 } };
    }

    /// Will simply not compile if the struct does not exist
    #[test]
    pub fn variables_name() {
        #[derive(Debug, Deserialize, Serialize, JsonVariables)]
        #[variables(variables_name = PersonDataNewVariables)]
        struct PersonData {
            name: String,
            age: u8
        }

        let _ = PersonDataNewVariables { variables: Default::default(), data: "".to_string() };
    }

    #[test]
    pub fn variables_field_name() {
        #[derive(Debug, Deserialize, Serialize, JsonVariables)]
        #[variables(variables_field_name = "vars")]
        struct PersonData {
            name: String,
            age: u8
        }

        let json = json!(
        {
            "vars": { "NAME": "John" },
            "person_data": { "name": "${NAME}", "age": 23 }
        });

        let person_data_variables: PersonDataVariables = serde_json::from_value(json).expect("Unable to parse json to variable");
        let x = person_data_variables.parse().expect("Unable to set variables");
        assert_eq!(x.name, "John");
        assert_eq!(x.age, 23); 
    }

    #[test]
    pub fn flatten_field_name() {
        #[derive(Debug, Deserialize, Serialize, JsonVariables)]
        #[variables(flatten_data_field)]
        struct PersonData {
            name: String,
            age: u8
        }

        let json = json!(
        {
            "variables": { "NAME": "John" },
            "name": "${NAME}", 
            "age": 23
        });

        let person_data_variables: PersonDataVariables = serde_json::from_value(json).expect("Unable to parse json to variable");
        let x = person_data_variables.parse().expect("Unable to set variables");
        assert_eq!(x.name, "John");
        assert_eq!(x.age, 23); 
    }
}
