use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

#[proc_macro_attribute]
pub fn testcase(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(item.clone()).unwrap();
    let id = uuid::Uuid::new_v4();
    let mut name = format!("{}", ast.ident);
    if name.starts_with("EDLog") {
        // Remove the EDLog prefix for the type name
        name = name.as_str()[5..].to_string();
    }
    let ty_name = Ident::new(&name, Span::call_site());
    let fn_name = Ident::new(
        &format!("test_case_for_{name}_{}", id.as_u128()),
        Span::call_site(),
    );
    let json_str = attr.to_string();

    let generated = quote! {
            #ast

            #[test]
            fn #fn_name() {
                use crate::log_line::EDLogLine;
                let str = #json_str;
                let line = serde_json::from_str::<EDLogLine>(str).expect("Should parse");

                assert!(matches!(line.event(),crate::log_line::EDLogEvent::#ty_name(_)));
            }
        };

    generated.into()
}

#[proc_macro_attribute]
pub fn testcase_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(item.clone()).unwrap();
    let id = uuid::Uuid::new_v4();
    let mut name = format!("{}", ast.ident);
    if name.starts_with("EDLog") {
        // Remove the EDLog prefix for the type name
        name = name.as_str()[5..].to_string();
    }
    let ty_name = Ident::new(&name, Span::call_site());
    let fn_name = Ident::new(
        &format!("test_case_for_{name}_{}", id.as_u128()),
        Span::call_site(),
    );
    let json_str = attr.to_string();

    let generated = quote! {
            #ast

            #[test]
            fn #fn_name() {
                let str = #json_str;
                let result = serde_json::from_str::<#ty_name>(str);
                assert!(result.is_ok(), "Should parse, got: {:?}", result);
            }
        };

    generated.into()
}
