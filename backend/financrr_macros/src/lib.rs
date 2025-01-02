use financrr_macros_impl::impls::app_errors::{app_error_implementation, AppErrorsInput};
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// The `app_errors` macro is used to define custom error types for your application.
///
/// # Required Arguments
///
/// 1. `status_code`: The HTTP status code associated with the error.
/// 2. `error_code`: The error code representing the specific error.
/// 3. `func_name`: The function name to generate the error.
///
/// # Optional Named Arguments
///
/// 1. `details`: A custom message for the error. Defaults to `None`.
/// 2. `argument`: The type of the reference argument. Defaults to `None`.
/// 3. `generate_response`: A boolean indicating whether to generate a response struct. Defaults to `true`.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust, no_run
/// use financrr_macros::app_errors;
///
/// app_errors!(
///     (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::DB_CUSTOM_ERROR, DbCustomError);
/// );
/// ```
///
/// This defines an error with the following properties:
/// - `status_code`: `StatusCode::INTERNAL_SERVER_ERROR`
/// - `error_code`: `ErrorCode::DB_CUSTOM_ERROR`
/// - `func_name`: `DbCustomError`
/// - `details`: `None`
/// - `argument`: `None`
/// - `generate_response`: `true`
///
/// ## With Reference Argument
///
/// ```rust, no_run
/// use financrr_macros::app_errors;
///
/// app_errors!(
///     (StatusCode::BAD_REQUEST, ErrorCode::INVALID_VERIFICATION_TOKEN, InvalidVerificationToken, argument=JsonReference);
/// );
/// ```
///
/// This defines an error with a reference argument of type `JsonReference`.
///
/// ## With Optional Reference Argument
///
/// ```rust, no_run
/// use financrr_macros::app_errors;
///
/// app_errors!(
///     (StatusCode::BAD_REQUEST, ErrorCode::INVALID_VERIFICATION_TOKEN, InvalidVerificationToken, argument=Option<JsonReference>);
/// );
/// ```
///
/// This defines an error with an optional reference argument of type `Option<JsonReference>`.
///
/// ## With Custom Details and Disabled Response Generation
///
/// ```rust, no_run
/// use financrr_macros::app_errors;
///
/// app_errors!(
///     (StatusCode::BAD_REQUEST, ErrorCode::INVALID_VERIFICATION_TOKEN, InvalidVerificationToken, details="Custom details", argument=String, generate_response=false);
/// );
/// ```
///
/// This defines an error with custom details and disables the generation of the response struct.
#[cfg(not(doctest))]
#[proc_macro]
pub fn app_errors(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as AppErrorsInput);

    app_error_implementation(input).into()
}
