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

#[proc_macro_derive(Extractable)]
pub fn extractable_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let ty_name = &ast.ident;

    let mut variant_name = format!("{}", ast.ident);
    if variant_name.starts_with("EDLog") {
        // Remove the EDLog prefix for the type name
        variant_name = variant_name.as_str()[5..].to_string();
    }
    let extractor: syn::Stmt = syn::parse_str(&format!(
        "if let crate::log_line::EDLogEvent::{variant_name}(loc) = event {{ Some(loc) }} else {{ None }}"
    ))
    .unwrap();

    let generated = quote! {
        impl crate::log_line::Extractable for #ty_name {
            fn extract(event: &crate::log_line::EDLogEvent) -> Option<&Self> {
                #extractor
            }
        }
    };
    generated.into()
}

#[proc_macro_derive(CodexCategorize, attributes(CodexCategory))]
pub fn codex_categorize_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let ty_name = &ast.ident;

    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => {
            return syn::Error::new_spanned(
                &ast.ident,
                "CodexCategorize can only be derived for enums",
            )
            .to_compile_error()
            .into();
        }
    };

    let parsed = variants.iter().map(|variant| {
        let mut category = None;
        let mut sub_category = None;

        let attr = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("CodexCategory"))
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    variant,
                    "each variant needs #[CodexCategory(category = ..., sub_category = ...)]",
                )
            })?;

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("category") {
                category = Some(meta.value()?.parse::<syn::Expr>()?);
                Ok(())
            } else if meta.path.is_ident("sub_category") {
                sub_category = Some(meta.value()?.parse::<syn::Expr>()?);
                Ok(())
            } else {
                Err(meta.error("expected `category` or `sub_category`"))
            }
        })?;

        Ok::<_, syn::Error>((
            &variant.ident,
            category.ok_or_else(|| syn::Error::new_spanned(variant, "missing `category`"))?,
            sub_category
                .ok_or_else(|| syn::Error::new_spanned(variant, "missing `sub_category`"))?,
        ))
    });

    let parsed = match parsed.collect::<Result<Vec<_>, _>>() {
        Ok(parsed) => parsed,
        Err(err) => return err.to_compile_error().into(),
    };

    let category_arms = parsed.iter().map(|(name, category, _)| {
        quote! { Self::#name => #category, }
    });

    let sub_category_arms = parsed.iter().map(|(name, _, sub_category)| {
        quote! { Self::#name => #sub_category, }
    });

    quote! {
        impl #ty_name {
            pub fn category(&self) -> CodexCategory {
                match self {
                    #(#category_arms)*
                }
            }

            pub fn sub_category(&self) -> CodexSubCategory {
                match self {
                    #(#sub_category_arms)*
                }
            }
        }
    }
    .into()
}
