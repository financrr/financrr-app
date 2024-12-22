use financrr_macros_impl::impls::app_errors::{app_error_implementation, AppErrorsInput};
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn app_errors(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as AppErrorsInput);

    app_error_implementation(input).into()
}
