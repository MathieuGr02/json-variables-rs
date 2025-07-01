use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Variables)]
pub fn derive_variables(input: TokenStream) -> TokenStream {
    /*
    let input = parse_macro_input!(input as DeriveInput);
    
    TokenStream::from(
        syn::Error::new(
            input.ident.span(), "Only structs can derive `Variables`"
        ).to_compile_error()
    );
    */
    input
}
