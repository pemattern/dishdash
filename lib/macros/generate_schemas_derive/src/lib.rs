use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(GenerateSchemas, attributes(schema))]
pub fn generate_schemas(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_read_only = input.attrs.iter().any(|attr| {
        if !attr.path().is_ident("schema") {
            return false;
        }

        let mut read_only = false;

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("read_only") {
                read_only = true;
            }

            Ok(())
        });

        read_only
    });
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(
                    name,
                    "GenerateSchemas only supports structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                name,
                "GenerateSchemas only supports structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };

    let mut response_fields = Vec::new();
    let mut response_field_names = Vec::new();
    let mut post_fields = Vec::new();
    let mut patch_fields = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let mut read = true;
        let mut write = true;
        let mut rename: Option<syn::LitStr> = None;

        for attr in &field.attrs {
            if !attr.path().is_ident("schema") {
                continue;
            }

            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    read = false;
                    write = false;
                } else if meta.path.is_ident("read_only") {
                    read = true;
                    write = false;
                } else if meta.path.is_ident("write_only") {
                    read = false;
                    write = true;
                } else if meta.path.is_ident("rename") {
                    rename = Some(meta.value()?.parse()?);
                }

                Ok(())
            });
        }

        if !read && !write {
            continue;
        }

        let schema_field_name = rename
            .as_ref()
            .map(|name| syn::Ident::new(&name.value(), name.span()))
            .unwrap_or_else(|| field_name.clone());

        if read {
            response_fields.push(quote! {
                pub #schema_field_name: #field_type
            });
            response_field_names.push((schema_field_name.clone(), field_name.clone()));
        }

        if write {
            post_fields.push(quote! {
                pub #schema_field_name: #field_type
            });

            patch_fields.push(quote! {
                pub #schema_field_name: Option<#field_type>
            });
        }
    }

    let response_name = syn::Ident::new(&format!("{}Schema", name), name.span());
    let post_name = syn::Ident::new(&format!("Post{}Schema", name), name.span());
    let patch_name = syn::Ident::new(&format!("Patch{}Schema", name), name.span());

    let response_assignments = response_field_names
        .iter()
        .map(|(schema_name, model_name)| {
            quote! {
                #schema_name: model.#model_name
            }
        });

    let write_schemas = if struct_read_only {
        quote! {}
    } else {
        quote! {
            #[derive(Debug, Clone, serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub struct #post_name {
                #(#post_fields,)*
            }

            #[derive(Debug, Clone, serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub struct #patch_name {
                #(#patch_fields,)*
            }
        }
    };

    quote! {
        pub mod schemas {
            use super::*;

            impl GenerateSchemas for #name {}

            #[derive(Debug, Clone, serde::Serialize)]
            #[serde(rename_all = "camelCase")]
            pub struct #response_name {
                #(#response_fields,)*
            }

            impl From<#name> for #response_name {
                fn from(model: #name) -> Self {
                    Self {
                        #(
                            #response_assignments,
                        )*
                    }
                }
            }

            #write_schemas
        }
    }
    .into()
}
