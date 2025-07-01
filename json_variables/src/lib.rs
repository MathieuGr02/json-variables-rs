//! # JSON Variables
//!
//! Variable system in json using serde json. Instead of repeating multiple definitions in your
//! JSON and bloating the file, define variables to easily reuse objects across your JSON file.
//! This type of system is especially usefull for config based systems where large configurations
//! in JSON are needed to define objects and behaviour in a system.
//!
//! A JSON like this
//!
//! ```json
//! [
//!     {
//!         "name": "Franklin",
//!         "country": {
//!             "name": "Netherlands",
//!             "abbreviation": "NL"
//!         }
//!     },
//!     {
//!         "name" "John",
//!         "country": {
//!             "name": "Netherlands",
//!             "abbreviation": "NL"
//!         }
//!     }
//! ]
//! ```
//!
//! could be simplified to 
//!
//! ```json
//! {
//!     "variables": {
//!         "NL": {
//!             "name": "Netherlands",
//!             "abbreviation": "NL"
//!         }
//!     },
//!     "data": 
//!     [
//!         {
//!             "name": "Franklin",
//!             "country": "${NL}"
//!         },
//!         {
//!             "name" "John",
//!             "country": "${NL}"
//!         }
//!     ]
//! }
//! ```
//!
//! Variables can also be defined inside ones own programm without defining it in the JSON. This is
//! done through the use of the serde_json `json!` macro or via a [`String`].
//!
//! ```
//! use serde_json::json;
//!
//! let data = r#"
//!     [
//!         {
//!             "name": "Franklin",
//!             "country": "${NL}"
//!         },
//!         {
//!             "name" "John",
//!             "country": "${NL}"
//!         }
//!     ]"#;
//!
//! let variables = json!(
//!     {
//!         "NL": {
//!             "name": "Netherlands",
//!             "abbreviation": "NL"
//!         }
//!     }
//! );
//! let variables = json_variables::from_json(variables).expect("Unable to parse json to variables");
//! let result = variables.replace(data); 
//! ```
//!
//! ```
//! let data = r#"
//!     [
//!         {
//!             "name": "Franklin",
//!             "country": "${NL}"
//!         },
//!         {
//!             "name" "John",
//!             "country": "${NL}"
//!         }
//!     ]"#;
//!
//! let variables = r#"
//!     { 
//!         "NL": { 
//!             "name": "Netherlands", 
//!             "abbreviation": "NL" 
//!         } 
//!     }
//! "#;
//! let variables = json_variables::from_str(variables).expect("Unable to parse string to variables");
//! dbg!(&variables);
//! let result = variables.replace(data); 
//! ```
//!
//! By default the matching pattern is ${\<variable\>}. This pattern also allows for value
//! accessing or array indexing through the `.` accessor.
//!
//! ```json
//! {
//!     "variables": {
//!         "NL": {
//!             "name": "Netherlands",
//!             "abbreviation": "NL"
//!         }
//!     },
//!     "data": 
//!     [
//!         {
//!             "name": "Franklin",
//!             "country": {
//!                 "name": "${NL.name}",
//!                 "abbreviation": "${NL.abbreviation}"
//!             }
//!         },
//!         {
//!             "name" "John",
//!             "country": "${NL}"
//!         }
//!     ]
//! }
//! ```

pub mod tree;
pub use tree::*;

pub mod variables;
pub use variables::*;

pub mod error;
pub use error::*;

pub mod traits;
pub use traits::*;
