use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};
use primitive_types::H160;
use std::str::FromStr;

#[proc_macro]
pub fn addr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let address = H160::from_str(input.value().as_str()).expect("the string can't be parsed as Address");
    let bytes: Vec<String> = address.as_fixed_bytes().into_iter().map(|b| format!("{}u8", b)).collect();
    format!("primitive_types::H160::from([{}])", bytes.join(",")).parse().unwrap()
}