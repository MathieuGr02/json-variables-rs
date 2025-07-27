#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use json_variables::JsonVariables;

    #[derive(Deserialize, Serialize, Debug)]
    #[derive(JsonVariables)]
    pub struct Object {
        pub name: String
    }

    #[test]
    pub fn parse() {
        let json = r#"
            {
                "variables": { "NAME": "John" },
                "object": { "name": "${NAME}" }
            }"#;

        let object: ObjectVariables = serde_json::from_str(&json).expect("Unable to parse string to object variable");
        let object = object.parse().expect("Unable to create object");
        assert_eq!(object.name, "John");
    }

    #[test]
    #[should_panic]
    pub fn incorrect_variable_parse() {
        let json = r#"
            {
                "variables": { "NAME": "John" },
                "object": { "name": "${TEST}" }
            }"#;

        let object: ObjectVariables = serde_json::from_str(json).expect("Unable to parse string to object variable");
        let _ = object.parse().expect("Unable to create object");
    }

    #[test]
    #[should_panic]
    pub fn incorrect_casing_parse() {
        let json = r#"
            {
                "variables": { "NAME": "John" },
                "Object": { "name": "${NAME}" }
            }"#;

        let object: ObjectVariables = serde_json::from_str(&json).expect("Unable to parse string to object variable");
        let _ = object.parse().expect("Unable to create object");
    }

    #[test]
    pub fn no_variables() {
        let json = r#"
            {
                "object": { "name": "John" }
            }"#;
        let object: ObjectVariables = serde_json::from_str(&json).expect("Unable to parse string to object variable");
        let object = object.parse().expect("Unable to create object");
        assert_eq!(object.name, "John");
    }

    #[test]
    #[should_panic]
    pub fn no_variables_with_variable_definition() {
        let json = r#"
            {
                "object": { "name": "${NAME}" }
            }"#;
        let object: ObjectVariables = serde_json::from_str(&json).expect("Unable to parse string to object variable");
        let _ = object.parse().expect("Unable to create object");
    }
}
