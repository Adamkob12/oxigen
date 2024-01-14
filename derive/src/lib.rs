mod ecs;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive_component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    ecs::component::derive(input).into()
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    ecs::resource::derive(input).into()
}
