use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};


#[derive(FromDeriveInput)]
#[darling(attributes(variables), forward_attrs(allow, doc, cfg))]
struct JsonVariablesOpt {
    /// Name of the temporary wrapper struct for deserialization
    wrapper_name: Option<syn::Ident>,
    /// Name of the variable struct for deseralization
    variables_name: Option<syn::Ident>,
    /// Field name of the data in the JSON file
    data_field_name: Option<String>,
    /// Field name of the variables in the JSON file
    variables_field_name: Option<String>,
    /// Flatten the data struct in the variables struct
    #[darling(default)]
    flatten_data_field: bool
}

fn to_snake_case(value: impl Into<String>) -> String {
    let cases = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let value = value.into();
    let chars = value.chars();
    let mut first = true;
    let mut snake_case_value = String::new();
    for mut c in chars {
        if cases.contains(&c) {
            c = c.to_lowercase().next().unwrap();
            if first {
                first = false;
            } else {
                snake_case_value.push('_');
            }
        }

        snake_case_value.push(c);
    }

    snake_case_value
}

#[proc_macro_derive(JsonVariables, attributes(variables))]
pub fn derive_variables(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref _data) = input.data {
        let opts = match JsonVariablesOpt::from_derive_input(&input) {
            Ok(opts) => opts,
            Err(err) => return err.write_errors().into()
        };

        let struct_type = input.ident.clone();

        let struct_name_snake_case = to_snake_case(struct_type.to_string());
        let struct_name_snake_case = format_ident!("{struct_name_snake_case}"); 
        
        let struct_wrapper_name = opts.wrapper_name
            .unwrap_or(format_ident!("{}Wrapper", input.ident.clone()));

        let struct_variable_name = opts.variables_name
            .unwrap_or(format_ident!("{}Variables", input.ident.clone()));

        let data_field_name = opts.data_field_name
            .unwrap_or( struct_name_snake_case.to_string().to_lowercase());

        let variables_field_name = opts.variables_field_name
            .unwrap_or("variables".to_string());


        let flatten = match opts.flatten_data_field {
            true => Some(quote!(#[serde(flatten)])),
            false => None 
        };

        return TokenStream::from(quote!(
            use serde::de::Error;

            #[derive(Debug)]
            pub struct #struct_variable_name {
                variables: json_variables::Variables,
                data: String,
            }

            #[derive(Debug, Deserialize, Serialize)]
            pub struct #struct_wrapper_name {
                #[serde(alias = #data_field_name)]
                #flatten
                #struct_name_snake_case: #struct_type
            }

            impl<'de> serde::Deserialize<'de> for #struct_variable_name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let mut data: serde_json::map::Map<String, serde_json::Value> = serde::Deserialize::deserialize(deserializer)?;
                    let mut variables = json_variables::Variables::default();
                    if let Some(var) = data.remove(#variables_field_name) {
                        variables = serde_json::from_value(var)
                            .map_err(D::Error::custom)?;
                    }

                    let data = serde_json::to_string(&data).map_err(D::Error::custom)?;

                    Ok(#struct_variable_name { variables, data })
                }
            }

            impl #struct_variable_name {
                pub fn parse(self) -> Result<#struct_type, serde_json::Error> {
                    let variables = self.variables;
                    let data = self.data;
                    let data = variables.replace(data);
                    let data: #struct_wrapper_name = serde_json::from_str(data.as_str())?;
                    Ok(data.#struct_name_snake_case)
                }
            }
        ));
    }
    
    TokenStream::from(
        syn::Error::new(
            input.ident.span(), "Only structs can derive `Variables`"
        ).to_compile_error()
    )
}
