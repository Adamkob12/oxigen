use quote::quote;
use syn::DeriveInput;

pub fn derive(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Resource for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&self) -> &mut dyn std::any::Any {
                self
            }
        }
    }
    .into();

    expanded
}

// pub trait Resource: Send + Sync + 'static {
//     fn as_any(&self) -> &dyn Any;
//     fn as_any_mut(&mut self) -> &mut dyn Any;
// }
