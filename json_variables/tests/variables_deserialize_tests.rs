#[cfg(test)]
mod tests {
    use json_variables::variables::Variables;
    use rstest::rstest;
    use serde_json::{Map, Value};

    #[rstest]
    #[case(r#"{ "NAME": "person" }"#)]
    #[case(r#"{ "NAME": "person", "AGE": 1 }"#)]
    #[case(r#"{ "NAME": "person", "EXISTS": true }"#)]
    #[case(r#"{ "NAME": "person", "COUNTRIES": ["NL", "CH"] }"#)]
    #[case(r#"{ "NAME": "person", "COUNTRY": { "short": "CH", "long": "Switzerland" }}"#)]
    pub fn variables_deserialize(#[case] var: &str) {
        let variables_1: Variables = serde_json::from_str(var).unwrap();
        let variables_2: Map<String, Value> = serde_json::from_str(var).unwrap();

        assert_eq!(variables_1.variables, variables_2);
    }

    #[rstest]
    #[case(r#"{ "NAME": "person" }"#)]
    #[case(r#"{ "NAME": "person", "AGE": 1 }"#)]
    #[case(r#"{ "NAME": "person", "EXISTS": true }"#)]
    #[case(r#"{ "NAME": "person", "COUNTRIES": ["NL", "CH"] }"#)]
    #[case(r#"{ "NAME": "person", "COUNTRY": { "short": "CH", "long": "Switzerland" }}"#)]
    pub fn variables_serialize(#[case] var: &str) {
        let variables_1: Variables = serde_json::from_str(var).unwrap();
        let variables_2: Map<String, Value> = serde_json::from_str(var).unwrap();

        let json_1 = serde_json::to_string(&variables_1).unwrap();
        let json_2 = serde_json::to_string(&variables_2).unwrap();

        assert_eq!(json_1, json_2);
    }
}
