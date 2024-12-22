use crate::utils::parsing::parse_bool;
use crate::utils::string::StringExt;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Expr, Token};

pub struct ErrorDefinition {
    docs: Vec<Attribute>,
    status_code: Expr,
    error_code: Expr,
    func_name: Ident,
    // Optional arguments
    details: Expr,
    reference_type: Option<syn::Type>,
    generate_response: bool,
}

impl Parse for ErrorDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse optional attributes (like #[...] docs)
        let docs = input.call(Attribute::parse_outer)?;

        let content;
        syn::parenthesized!(content in input);

        // Parse the 3 required parts inside the parentheses
        let status_code: Expr = content.parse()?;
        content.parse::<Token![,]>()?;

        let error_code: Expr = content.parse()?;
        content.parse::<Token![,]>()?;

        let func_name: Ident = content.parse()?;

        // Default values for optional named arguments
        let mut details: Expr = syn::parse_quote! { #error_code.message };
        let mut reference_type = None;
        let mut generate_response = true;

        // Parse optional named arguments
        while content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
            let ident: Ident = content.parse()?;
            content.parse::<Token![=]>()?;
            match ident.to_string().as_str() {
                "details" => details = content.parse()?,
                "argument" => reference_type = Some(content.parse()?),
                "generate_response" => generate_response = parse_bool(&content)?,
                _ => return Err(syn::Error::new(ident.span(), "Unexpected named argument")),
            }
        }

        input.parse::<Token![;]>()?; // Consume the trailing semicolon

        Ok(Self {
            docs,
            status_code,
            error_code,
            func_name,
            details,
            reference_type,
            generate_response,
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

    let structs = input
        .errors
        .iter()
        .filter(|error| error.generate_response)
        .map(|error| {
            let status_code = &error.status_code;
            let details = &error.details;
            let func_name = &error.func_name;

            let example = if error.reference_type.is_some() {
                quote! { AppError::#func_name(Default::default()) }
            } else {
                quote! { AppError::#func_name() }
            };

            let struct_name = Ident::new(&format!("{}Response", func_name), func_name.span());

            quote! {
                #[derive(IntoResponses)]
                #[response(
                    status = #status_code,
                    description = #details,
                    example = json!(#example),
                    content_type = "application/json"
                )]
                #[allow(dead_code)]
                pub struct #struct_name(#[to_schema] AppError);
            }
        });

    let expanded = quote! {
        impl AppError {
            #(#functions)*
        }

        #(#structs)*
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
            (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::DB_CUSTOM_ERROR, DbCustomError);
        };

        let generated = app_error_implementation(input);
        let expected = quote! {
            impl AppError {
                #[allow(non_snake_case)]
                pub(crate) fn DbCustomError() -> Self {
                    Self {
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        error_code: ErrorCode::DB_CUSTOM_ERROR,
                        details: String::from(ErrorCode::DB_CUSTOM_ERROR.message),
                        reference: None,
                    }
                }
            }

            #[derive(IntoResponses)]
            #[response(
                status = StatusCode::INTERNAL_SERVER_ERROR,
                description = ErrorCode::DB_CUSTOM_ERROR.message,
                example = json!(AppError::DbCustomError()),
                content_type = "application/json"
            )]
            #[allow(dead_code)]
            pub struct DbCustomErrorResponse(#[to_schema] AppError);
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn test_single_error_definition_with_reference() {
        let input: AppErrorsInput = parse_quote! {
            (StatusCode::BAD_REQUEST, ErrorCode::INVALID_VERIFICATION_TOKEN, InvalidVerificationToken, argument=JsonReference);
        };

        let generated = app_error_implementation(input);
        let expected = quote! {
            impl AppError {
                #[allow(non_snake_case)]
                pub(crate) fn InvalidVerificationToken(reference: JsonReference) -> Self {
                    Self {
                        status_code: StatusCode::BAD_REQUEST,
                        error_code: ErrorCode::INVALID_VERIFICATION_TOKEN,
                        details: String::from(ErrorCode::INVALID_VERIFICATION_TOKEN.message),
                        reference: JsonReference::new_with_default_none(&reference),
                    }
                }
            }

            #[derive(IntoResponses)]
            #[response(
                status = StatusCode::BAD_REQUEST,
                description = ErrorCode::INVALID_VERIFICATION_TOKEN.message,
                example = json!(AppError::InvalidVerificationToken(Default::default())),
                content_type = "application/json"
            )]
            #[allow(dead_code)]
            pub struct InvalidVerificationTokenResponse(#[to_schema] AppError);
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn test_single_error_definition_with_optional_reference() {
        let input: AppErrorsInput = parse_quote! {
            (StatusCode::BAD_REQUEST, ErrorCode::INVALID_VERIFICATION_TOKEN, InvalidVerificationToken, argument=Option<JsonReference>);
        };

        let generated = app_error_implementation(input);
        let expected = quote! {
            impl AppError {
                #[allow(non_snake_case)]
                pub(crate) fn InvalidVerificationToken(reference: Option<JsonReference>) -> Self {
                    Self {
                        status_code: StatusCode::BAD_REQUEST,
                        error_code: ErrorCode::INVALID_VERIFICATION_TOKEN,
                        details: String::from(ErrorCode::INVALID_VERIFICATION_TOKEN.message),
                        reference: reference,
                    }
                }
            }

            #[derive(IntoResponses)]
            #[response(
                status = StatusCode::BAD_REQUEST,
                description = ErrorCode::INVALID_VERIFICATION_TOKEN.message,
                example = json!(AppError::InvalidVerificationToken(Default::default())),
                content_type = "application/json"
            )]
            #[allow(dead_code)]
            pub struct InvalidVerificationTokenResponse(#[to_schema] AppError);
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn test_multiple_error_definitions() {
        let input: AppErrorsInput = parse_quote! {
            (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::DB_CUSTOM_ERROR, DbCustomError);
            (StatusCode::UNAUTHORIZED, ErrorCode::INVALID_JWT, InvalidJwt);
        };

        let generated = app_error_implementation(input);
        let expected = quote! {
            impl AppError {
                #[allow(non_snake_case)]
                pub(crate) fn DbCustomError() -> Self {
                    Self {
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        error_code: ErrorCode::DB_CUSTOM_ERROR,
                        details: String::from(ErrorCode::DB_CUSTOM_ERROR.message),
                        reference: None,
                    }
                }

                #[allow(non_snake_case)]
                pub(crate) fn InvalidJwt() -> Self {
                    Self {
                        status_code: StatusCode::UNAUTHORIZED,
                        error_code: ErrorCode::INVALID_JWT,
                        details: String::from(ErrorCode::INVALID_JWT.message),
                        reference: None,
                    }
                }
            }

            #[derive(IntoResponses)]
            #[response(
                status = StatusCode::INTERNAL_SERVER_ERROR,
                description = ErrorCode::DB_CUSTOM_ERROR.message,
                example = json!(AppError::DbCustomError()),
                content_type = "application/json"
            )]
            #[allow(dead_code)]
            pub struct DbCustomErrorResponse(#[to_schema] AppError);

            #[derive(IntoResponses)]
            #[response(
                status = StatusCode::UNAUTHORIZED,
                description = ErrorCode::INVALID_JWT.message,
                example = json!(AppError::InvalidJwt()),
                content_type = "application/json"
            )]
            #[allow(dead_code)]
            pub struct InvalidJwtResponse(#[to_schema] AppError);
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn test_error_definition_with_named_arguments() {
        let input: AppErrorsInput = parse_quote! {
            (StatusCode::BAD_REQUEST, ErrorCode::INVALID_VERIFICATION_TOKEN, InvalidVerificationToken, details="Custom details", argument=String, generate_response=false);
        };

        let generated = app_error_implementation(input);
        let expected = quote! {
            impl AppError {
                #[allow(non_snake_case)]
                pub(crate) fn InvalidVerificationToken(reference: String) -> Self {
                    Self {
                        status_code: StatusCode::BAD_REQUEST,
                        error_code: ErrorCode::INVALID_VERIFICATION_TOKEN,
                        details: String::from("Custom details"),
                        reference: JsonReference::new_with_default_none(&reference),
                    }
                }
            }
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }
}
