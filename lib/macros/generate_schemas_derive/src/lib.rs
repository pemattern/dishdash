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
    let mut update_fields = Vec::new();
    let mut update_assignments = Vec::new();

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
                #(#helper_attrs)*
                pub #schema_field_name: #field_type
            });

            let (patch_type, nullable) = unwrap_option_type(field_type);
            let optional_field = if nullable {
                let ts_type = syn::LitStr::new(
                    &format!("Option<{}>", quote!(#patch_type)),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    #[ts(optional = nullable, as = #ts_type)]
                    pub #field_name: Option<#field_type>
                }
            } else {
                quote! {
                    #[ts(optional)]
                    pub #field_name: Option<#field_type>
                }
            };

            update_fields.push(optional_field);

            let conversion = if nullable {
                quote! {
                    match schema.#schema_field_name {
                        crate::requests::patch_field::PatchField::Omitted => None,
                        crate::requests::patch_field::PatchField::Cleared => Some(None),
                        crate::requests::patch_field::PatchField::Value(value) => Some(Some(value)),
                    }
                }
            } else {
                quote! {
                    match schema.#schema_field_name {
                        crate::requests::patch_field::PatchField::Omitted => None,
                        crate::requests::patch_field::PatchField::Cleared => {
                            return Err(crate::error::AppError::InvalidData);
                        }
                        crate::requests::patch_field::PatchField::Value(value) => Some(value),
                    }
                }
            };

            update_assignments.push(quote! {
                #field_name: #conversion
            });

            patch_fields.push(quote! {
                #[serde(default)]
                #(#helper_attrs)*
                pub #schema_field_name: crate::requests::patch_field::PatchField<#patch_type>
            });
        }
    }

    let response_name = syn::Ident::new(&format!("{}Schema", name), name.span());
    let post_name = syn::Ident::new(&format!("Post{}Schema", name), name.span());
    let patch_name = syn::Ident::new(&format!("Patch{}Schema", name), name.span());
    let patch_name_lit = syn::LitStr::new(&patch_name.to_string(), patch_name.span());
    let updates_name = syn::Ident::new(&format!("{}Updates", name), name.span());

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

            #[derive(serde::Deserialize, validator::Validate)]
            #[serde(rename_all = "camelCase")]
            pub struct #patch_name {
                #(#patch_fields,)*
            }

            #[derive(ts_rs::TS)]
            #[ts(export, export_to = lib::BINDINGS_PATH, rename = #patch_name_lit, rename_all = "camelCase")]
            pub struct #updates_name {
                #(#update_fields,)*
            }

            impl TryFrom<#patch_name> for #updates_name {
                type Error = crate::error::AppError;

                fn try_from(schema: #patch_name) -> Result<Self, Self::Error> {
                    Ok(Self {
                        #(#update_assignments,)*
                    })
                }
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
