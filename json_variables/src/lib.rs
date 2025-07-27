//! # JSON Variables
//!
//! Variable system in json using serde json. Instead of repeating multiple definitions in your
//! JSON and bloating the file, define variables to easily reuse objects across your JSON file.
//! This type of system is especially useful for config based systems where large configurations
//! in JSON are needed to define objects and behaviour in a system.
//!
//! A JSON like this
//!
//! ```json
//! [
//!     {
//!         "name": "Franklin",
//!         "country": {
//!             "long": "Netherlands",
//!             "short": "NL"
//!         }
//!     },
//!     {
//!         "long" "John",
//!         "country": {
//!             "long": "Netherlands",
//!             "short": "NL"
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
//!             "long": "Netherlands",
//!             "short": "NL"
//!         }
//!     },
//!     "data": 
//!     [
//!         {
//!             "name": "Franklin",
//!             "country": "${NL}"
//!         },
//!         {
//!             "name": "John",
//!             "country": "${NL}"
//!         }
//!     ]
//! }
//! ```
//!
//! Variables can also be defined inside ones own program without defining it in the JSON. This is
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
//!             "name": "John",
//!             "country": "${NL}"
//!         }
//!     ]"#;
//!
//! let variables = json!(
//!     {
//!         "NL": {
//!             "long": "Netherlands",
//!             "short": "NL"
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
//!             "name": "John",
//!             "country": "${NL}"
//!         }
//!     ]"#;
//!
//! let variables = r#"
//!     { 
//!         "NL": { 
//!             "long": "Netherlands", 
//!             "short": "NL" 
//!         } 
//!     }
//! "#;
//! let variables = json_variables::from_str(variables).expect("Unable to parse string to variables");
//! dbg!(&variables);
//! let result = variables.replace(data); 
//! ```
//!
//! By default, the matching pattern is ${\<variable\>}. This pattern also allows for value
//! accessing or array indexing through the `.` accessor.
//!
//! ```json
//! {
//!     "variables": {
//!         "NL": {
//!             "long": "Netherlands",
//!             "short": "NL"
//!         }
//!     },
//!     "data": 
//!     [
//!         {
//!             "name": "Franklin",
//!             "country": {
//!                 "long": "${NL.long}",
//!                 "short": "${NL.short}"
//!             }
//!         },
//!         {
//!             "name": "John",
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

pub use derive_json_variables::*;