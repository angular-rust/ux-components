extern crate proc_macro;

use syn;
use quote::quote;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let output = input.sig.output;
    let block = input.block;
    let output = quote! {
        fn main() #output {
            fn app_main(logger: Logger, config: Config, signal: microservice::Signal) #output {
                #block
            }
            microservice::microservice_run(app_main)
        }
    };
    output.into()
}
