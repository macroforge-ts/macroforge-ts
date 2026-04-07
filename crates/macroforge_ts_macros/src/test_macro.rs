use proc_macro::TokenStream;

#[proc_macro]
pub fn test_macro(_: TokenStream) -> TokenStream {
    let target = std::env::var("TARGET").unwrap_or_else(|_| "NOT_SET".to_string());
    format!("pub const TARGET_ENV: &str = \"{}\";", target).parse().unwrap()
}
