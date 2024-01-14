use quote::quote;
use syn::DeriveInput;

pub fn derive(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Component for #name #ty_generics #where_clause {
            fn comp_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<Self>()
            }

            fn layout(&self) -> std::alloc::Layout {
                std::alloc::Layout::new::<Self>()
            }

            fn drop_fn(&self) -> Option<unsafe fn(OwningPtr<'_>)> {
                Some(drop_ptr::<Self>)
            }

            fn name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    }
    .into();

    expanded
}

// fn as_any(&self) -> &dyn std::any::Any;
// fn comp_id(&self) -> ComponentId;
// fn layout(&self) -> std::alloc::Layout;
// fn drop_fn(&self) -> Option<unsafe fn(OwningPtr<'_>)>;
// fn name(&self) -> &'static str;
