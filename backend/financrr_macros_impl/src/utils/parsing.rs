use syn::Expr;
use syn::parse::ParseStream;
use syn::spanned::Spanned;

pub(crate) fn parse_bool(input: ParseStream) -> syn::Result<bool> {
    let expr: Expr = input.parse()?;
    match expr {
        Expr::Lit(expr_lit) => {
            if let syn::Lit::Bool(lit_bool) = expr_lit.lit {
                Ok(lit_bool.value)
            } else {
                Err(syn::Error::new(expr_lit.span(), "Expected a boolean literal"))
            }
        }
        _ => Err(syn::Error::new(expr.span(), "Expected a boolean literal")),
    }
}
