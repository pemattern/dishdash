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

        let mut helper_attrs = Vec::new();

        for attr in &field.attrs {
            if attr.path().is_ident("validate") {
                helper_attrs.push(attr);
            }
        }
        let mut read = true;
        let mut write = true;
        let mut read_rename: Option<syn::LitStr> = None;
        let mut write_rename: Option<syn::LitStr> = None;

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
                } else if meta.path.is_ident("read_rename") {
                    read_rename = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("write_rename") {
                    write_rename = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("rename") {
                    let rename: syn::LitStr = meta.value()?.parse()?;
                    read_rename = Some(rename.clone());
                    write_rename = Some(rename);
                }

                Ok(())
            });
        }

        if !read && !write {
            continue;
        }

        if read {
            let read_field_name = read_rename
                .as_ref()
                .map(|name| syn::Ident::new(&name.value(), name.span()))
                .unwrap_or_else(|| field_name.clone());

            response_fields.push(quote! {
                pub #read_field_name: #field_type
            });
            response_field_names.push((read_field_name.clone(), field_name.clone()));
        }

        if write {
            let write_field_name = write_rename
                .as_ref()
                .map(|name| syn::Ident::new(&name.value(), name.span()))
                .unwrap_or_else(|| field_name.clone());

            post_fields.push(quote! {
                #(#helper_attrs)*
                pub #write_field_name: #field_type
            });

            let (patch_type, nullable) = unwrap_option_type(field_type);
            let optional_field = if nullable {
                let ts_type = syn::LitStr::new(
                    &format!("Option<{}>", quote!(#patch_type)),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    #[ts(optional = nullable, as = #ts_type)]
                    pub #write_field_name: Option<#field_type>
                }
            } else {
                quote! {
                    #[ts(optional)]
                    pub #write_field_name: Option<#field_type>
                }
            };

            patch_fields.push(quote! {
                #[serde(default, deserialize_with = "crate::models::deserialize_double_option")]
                #(#helper_attrs)*
                #optional_field
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
            #[derive(serde::Deserialize, ts_rs::TS, validator::Validate)]
            #[serde(rename_all = "camelCase")]
            #[ts(export, export_to = lib::BINDINGS_PATH)]
            pub struct #post_name {
                #(#post_fields,)*
            }

            #[derive(serde::Deserialize, ts_rs::TS, validator::Validate)]
            #[serde(rename_all = "camelCase")]
            #[ts(export, export_to = lib::BINDINGS_PATH)]
            pub struct #patch_name {
                #(#patch_fields,)*
            }
        }
    };

    quote! {
        impl GenerateSchemas for #name {}

        #[derive(serde::Serialize, ts_rs::TS)]
        #[serde(rename_all = "camelCase")]
        #[ts(export, export_to = lib::BINDINGS_PATH)]
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
    .into()
}

fn unwrap_option_type(ty: &syn::Type) -> (&syn::Type, bool) {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                        return (inner, true);
                    }
                }
            }
        }
    }

    (ty, false)
}
