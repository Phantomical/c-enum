use proc_macro::TokenStream;

mod c_enum;

#[proc_macro_attribute]
pub fn c_enum(attr: TokenStream, mut input: TokenStream) -> TokenStream {
    match crate::c_enum::expand(attr.into(), input.clone().into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => {
            input.extend(TokenStream::from(e.to_compile_error()));
            input
        }
    }
}
