use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Model)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => {
            return syn::Error::new_spanned(name, "Model can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let has_id = match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .any(|field| field.ident.as_ref().is_some_and(|ident| ident == "id")),
        _ => false,
    };

    if !has_id {
        return syn::Error::new_spanned(name, "Model requires a named `id` field")
            .to_compile_error()
            .into();
    }

    quote! {
        impl ::lib::Model for #name {
            fn id(&self) -> ::uuid::Uuid {
                self.id
            }
        }
    }
    .into()
}
