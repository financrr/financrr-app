use crate::utils::string::StringExt;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Expr, Token};

pub struct ErrorDefinition {
    docs: Vec<Attribute>,
    status_code: Expr,
    error_code: Expr,
    details: Expr,
    func_name: Ident,
    reference_type: Option<syn::Type>,
}

impl Parse for ErrorDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse optional attributes (like #[...] docs)
        let docs = input.call(Attribute::parse_outer)?;

        let content;
        syn::parenthesized!(content in input);

        // Parse the 5 parts inside the parentheses
        let status_code: Expr = content.parse()?;
        content.parse::<Token![,]>()?;

        let error_code: Expr = content.parse()?;
        content.parse::<Token![,]>()?;

        let details: Expr = content.parse()?;
        content.parse::<Token![,]>()?;

        let func_name: Ident = content.parse()?;

        let reference_type = if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
            Some(content.parse()?)
        } else {
            None
        };

        input.parse::<Token![;]>()?; // Consume the trailing semicolon

        Ok(Self {
            docs,
            status_code,
            error_code,
            details,
            func_name,
            reference_type,
        })
    }
}

/// Represents the entire macro input containing multiple error definitions
pub struct AppErrorsInput {
    errors: Vec<ErrorDefinition>,
}

impl Parse for AppErrorsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors = Vec::new();

        while !input.is_empty() {
            let error: ErrorDefinition = input.parse()?; // Parse each error definition
            errors.push(error);
        }

        Ok(Self { errors })
    }
}

pub fn app_error_implementation(input: AppErrorsInput) -> TokenStream {
    let functions = input.errors.iter().map(|error| {
        let docs = &error.docs;
        let status_code = &error.status_code;
        let error_code = &error.error_code;
        let details = &error.details;
        let func_name = &error.func_name;

        let reference_code = if let Some(reference_type) = &error.reference_type {
            if reference_type.to_token_stream().to_string().remove_whitespaces() == "Option<JsonReference>" {
                quote! { reference }
            } else {
                quote! { JsonReference::new_with_default_none(&reference) }
            }
        } else {
            quote! { None }
        };

        let func_args = if let Some(reference_type) = &error.reference_type {
            quote! { (reference: #reference_type) }
        } else {
            quote! { () }
        };

        quote! {
            #(#docs)*
            #[allow(non_snake_case)]
            pub(crate) fn #func_name #func_args -> Self {
                Self {
                    status_code: #status_code,
                    error_code: #error_code,
                    details: String::from(#details),
                    reference: #reference_code,
                }
            }
        }
    });

    let expanded = quote! {
        impl AppError {
            #(#functions)*
        }
    };

    expanded
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_single_error_definition_without_reference() {
        let input: AppErrorsInput = parse_quote! {
            (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::DB_CUSTOM_ERROR, "A custom DB error occurred.", DbCustomError);
        };

        let generated = app_error_implementation(input);
        let expected = quote! {
            impl AppError {
                #[allow(non_snake_case)]
                pub(crate) fn DbCustomError() -> Self {
                    Self {
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        error_code: ErrorCode::DB_CUSTOM_ERROR,
                        details: String::from("A custom DB error occurred."),
                        reference: None,
                    }
                }
            }
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn test_single_error_definition_with_reference() {
        let input: AppErrorsInput = parse_quote! {
            (StatusCode::BAD_REQUEST, ErrorCode::INVALID_VERIFICATION_TOKEN, "Invalid verification token.", InvalidVerificationToken, JsonReference);
        };

        let generated = app_error_implementation(input);
        let expected = quote! {
            impl AppError {
                #[allow(non_snake_case)]
                pub(crate) fn InvalidVerificationToken(reference: JsonReference) -> Self {
                    Self {
                        status_code: StatusCode::BAD_REQUEST,
                        error_code: ErrorCode::INVALID_VERIFICATION_TOKEN,
                        details: String::from("Invalid verification token."),
                        reference: JsonReference::new_with_default_none(&reference),
                    }
                }
            }
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn test_single_error_definition_with_optional_reference() {
        let input: AppErrorsInput = parse_quote! {
            (StatusCode::BAD_REQUEST, ErrorCode::INVALID_VERIFICATION_TOKEN, "Invalid verification token.", InvalidVerificationToken, Option<JsonReference>);
        };

        let generated = app_error_implementation(input);
        let expected = quote! {
            impl AppError {
                #[allow(non_snake_case)]
                pub(crate) fn InvalidVerificationToken(reference: Option<JsonReference>) -> Self {
                    Self {
                        status_code: StatusCode::BAD_REQUEST,
                        error_code: ErrorCode::INVALID_VERIFICATION_TOKEN,
                        details: String::from("Invalid verification token."),
                        reference: reference,
                    }
                }
            }
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn test_multiple_error_definitions() {
        let input: AppErrorsInput = parse_quote! {
            (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::DB_CUSTOM_ERROR, "A custom DB error occurred.", DbCustomError);
            (StatusCode::UNAUTHORIZED, ErrorCode::INVALID_JWT, "An invalid JWT was provided.", InvalidJwt);
        };

        let generated = app_error_implementation(input);
        let expected = quote! {
            impl AppError {
                #[allow(non_snake_case)]
                pub(crate) fn DbCustomError() -> Self {
                    Self {
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        error_code: ErrorCode::DB_CUSTOM_ERROR,
                        details: String::from("A custom DB error occurred."),
                        reference: None,
                    }
                }

                #[allow(non_snake_case)]
                pub(crate) fn InvalidJwt() -> Self {
                    Self {
                        status_code: StatusCode::UNAUTHORIZED,
                        error_code: ErrorCode::INVALID_JWT,
                        details: String::from("An invalid JWT was provided."),
                        reference: None,
                    }
                }
            }
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }
}
