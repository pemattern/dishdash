use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Payload, attributes(payload))]
pub fn generate_dtos(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    if !matches!(input.vis, syn::Visibility::Inherited) {
        return syn::Error::new_spanned(&input.vis, "Payload structs must have private visibility")
            .to_compile_error()
            .into();
    }

    let name_str = name.to_string();
    let Some(base_name) = name_str.strip_suffix("Payload") else {
        return syn::Error::new_spanned(
            name,
            "Payload structs must have a name ending in `Payload`",
        )
        .to_compile_error()
        .into();
    };

    if base_name.is_empty() {
        return syn::Error::new_spanned(
            name,
            "Payload structs must have a name before the `Payload` suffix",
        )
        .to_compile_error()
        .into();
    }

    let base_name = syn::Ident::new(base_name, name.span());

    let struct_read_only = input.attrs.iter().any(|attr| {
        if !attr.path().is_ident("payload") {
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

    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(
                    name,
                    "Payload only supports structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                name,
                "Payload only supports structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };

    let mut get_fields = Vec::new();
    let mut get_field_names = Vec::new();
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
            if !attr.path().is_ident("payload") {
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

            get_fields.push(quote! {
                pub #read_field_name: #field_type
            });
            get_field_names.push((read_field_name.clone(), field_name.clone()));
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
                #[serde(default, deserialize_with = "crate::payloads::deserialize_double_option")]
                #(#helper_attrs)*
                #optional_field
            });
        }
    }

    let get_name = syn::Ident::new(&format!("{}Response", base_name), base_name.span());
    let post_name = syn::Ident::new(&format!("Create{}Request", base_name), base_name.span());
    let patch_name = syn::Ident::new(&format!("Update{}Request", base_name), base_name.span());

    let write_payloads = if struct_read_only {
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
        impl lib::Payload for #get_name {}

        #[derive(serde::Serialize, ts_rs::TS)]
        #[serde(rename_all = "camelCase")]
        #[ts(export, export_to = lib::BINDINGS_PATH)]
        pub struct #get_name {
            #(#get_fields,)*
        }

        #write_payloads
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
