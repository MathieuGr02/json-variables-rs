use regex::Regex;
use serde::{de::Error, ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

use super::tree::VariableTree;


/// The `Variables` struct. Holds the variables of the json file in a `serde_json::Value::Map`
/// object
#[derive(Debug)]
pub struct Variables {
    pub variables: Map<String, Value>,
    pub pattern: Regex,
    tree: VariableTree
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VariablesDeserializer {
    #[serde(flatten)]
    variables: Map<String, Value>,
    #[serde(default = "default_pattern")]
    #[serde(deserialize_with = "deserialize_regex")]
    #[serde(serialize_with = "serialize_regex")]
    pattern: Regex,
}

// Manual implementation to auto create tree on deserialization
impl<'de> Deserialize<'de> for Variables {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        let helper  = VariablesDeserializer::deserialize(deserializer)?;

        Ok(Variables::new(helper.variables, Some(helper.pattern.to_string())).unwrap())
    }
}

impl Serialize for Variables {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
     
        let mut map = serializer.serialize_map(None)?;
        for (k, v) in self.variables.iter() {
            map.serialize_entry(k, v)?;
        }

        map.end()
    }
}




/// Create type variables from str
///
/// # Examples 
/// ```
/// let json = r#"{ "${NAME}": "rust" }"#;
/// let variables = json_variables::from_str(json).expect("Unable to parse string to variables");
/// ```
pub fn from_str(json: &str) -> Result<Variables, serde_json::Error> {
    let mut variables: Variables = serde_json::from_str(json)?;
    let tree = Variables::create_tree(&variables.variables);
    variables.tree = tree;
    Ok(variables)
}

/// Create type variables from serde_json `json!()` macro
///
/// # Examples 
/// ```
/// use serde_json::json;
///
/// let json = json!({ "${NAME}": "rust" });
/// let variables = json_variables::from_json(json).expect("Unable to parse json to variables");
/// ```
pub fn from_json(json: Value) -> Result<Variables, super::Error> {
    let json = json.as_object();
    if let Some(json) = json {
        return Variables::new(json.to_owned(), None);
    }
    
    Err(super::Error::InvalidJson)
}

pub fn deserialize_regex<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    let pattern = String::deserialize(deserializer)?;
    Regex::new(&pattern).map_err(Error::custom)
}

pub fn serialize_regex<S>(regex: &Regex, s: S) -> Result<S::Ok, S::Error> 
where
    S: Serializer,
{
    s.serialize_str(regex.as_str())
}

pub fn default_pattern() -> Regex {
    // Matches the pattern ${<variable(.variable>)} where (.variable) is a possible "infinite"
    // accessing of variables inside a map
    Regex::new(r"\$\{([a-zA-Z0-9_.]+)\}").unwrap()
}

impl Variables {
    pub fn new(variables: Map<String, Value>, pattern: Option<String>) -> Result<Variables, super::Error> {
        let var_pattern;
        if let Some(pattern) = pattern {
            var_pattern = Regex::new(&pattern).map_err(|_| super::Error::InvalidPattern(pattern))?;
        }
        else {
            var_pattern = default_pattern();
        }


        let tree = Self::create_tree(&variables);

        Ok(
            Variables 
            { 
                pattern: var_pattern,
                variables,
                tree,
            }
        )
    }
}

impl Variables {
    /// Generate the redundant variable tree
    fn create_tree(variables: &Map<String, Value>) -> VariableTree {
        let mut tree = VariableTree::new(Value::Object(variables.clone()));
        Self::insert_variables(0, variables.clone(), &mut tree);
        tree
    }

    /// Recursion to create the variable tree 
    fn insert_variables(parent: usize, value: Map<String, Value>, tree: &mut VariableTree) {
        for (key, value) in value {
            let id = tree.add_child(parent, key, value.clone());
            // If value itself is a map, repeat recursion 
            if value.is_object() {
                let map = value.as_object().unwrap().to_owned();
                Self::insert_variables(id, map, tree);
            }
            if value.is_array() {
                let mut map = Map::new();
                for (key, value) in value.as_array().unwrap().iter().enumerate() {
                    map.insert(format!("{}", key), value.to_owned());
                }
                Self::insert_variables(id, map, tree);
            }
        }
    }

    /// Replace mentions of variables of the pattern ${<variable>} with their true value as given
    /// in the variable tree
    pub fn replace(&self, config: impl Into<String>) -> String {
        let mut config = config.into();
        // Get all pattern matches
        let mut variables = Vec::<(String, String)>::new();
        for variable in self.pattern.captures_iter(&config) {
            // Index corresponds to the <variable> inside the pattern ${<variable>}
            variables.push((variable[0].to_string(), variable[1].to_string()));
        }

        // Iterate over all pattern matches, get the value and replace all mentions of that
        // variable with its value
        for (pattern_variable, variable) in variables {
            let value = self.tree.get_variable_value(&variable);
            if let Some(value) = value {
                // Replace all "pattern(<variable>)" references
                let pattern_variable_quote = format!("\"{pattern_variable}\"");
                while config.contains(&pattern_variable_quote) {
                    config = config.replace(&pattern_variable_quote, &format!("{}", value));
                }

                let mut value_string = value.to_string();
                // If value is serde_json::Value::String, then remove quotation marks because at
                // this point, the variable is concatinated with strings, therefor the quotation
                // marks are not needed
                if value.is_string() {
                    value_string = value_string.replace("\"", "");
                }
                // Replace all "...pattern(var)..." references
                while config.contains(&pattern_variable) {
                    config = config.replace(&pattern_variable, &value_string);
                }
            }
            else {
                panic!("Variable {} called but neved defined", variable);
            }
        }

        config
    }
}
