use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

mod application;

use application::getters::expand_getters;

//
// Application related things
//

#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_getters(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Setters)]
pub fn setters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_getters(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Application)]
pub fn writable_template_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    let expanded = quote! {
      impl #name {
        fn run() -> #name {
            application::init();
            let app = Self::new();
            application::run();
            app
        }

        fn quit() {
            application::quit()
        }
      }
    };

    TokenStream::from(expanded)
}

//
// Inspect related things
//

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
