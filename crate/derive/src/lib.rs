use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, ExprLit, Lit, Meta, MetaNameValue, Variant, parse_macro_input, Error};

/// Derive macro that generates `name()` and `from_str()` methods for enums
/// based on `#[type_name = "..."]` attributes on the variants.
///
/// ### Example
/// ```
/// use slang_derive::NamedEnum;
///
/// #[derive(Debug, NamedEnum)]
/// enum MyEnum {
///  #[name = "first_variant"]
///  First,
///  #[name = "second_variant"]
///  Second,
///  Third, // Implicit name: "third"
/// }
/// ```
#[proc_macro_derive(NamedEnum, attributes(name))]
pub fn derive_named_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let variants = if let Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        return Error::new_spanned(
            &input.ident,
            "NamedEnum can only be derived for enums"
        ).to_compile_error().into();
    };
    let variant_mappings = variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let string_name = match extract_name_attribute(variant) {
                Ok(Some(name)) => name,
                Ok(None) => variant_name.to_string().to_lowercase(),
                Err(err) => return Err(err),
            };
            Ok((variant_name, string_name))
        })
        .collect::<Result<Vec<_>, Error>>();

    let variant_mappings = match variant_mappings {
        Ok(mappings) => mappings,
        Err(err) => return err.to_compile_error().into(),
    };

    let type_name_arms = variant_mappings.iter().map(|(variant_name, string_name)| {
        quote! {
            #enum_name::#variant_name => #string_name
        }
    });

    let from_str_arms = variant_mappings.iter().map(|(variant_name, string_name)| {
        quote! {
            #string_name => Some(#enum_name::#variant_name)
        }
    });

    let expanded = quote! {
        impl #enum_name {
            pub const fn name(&self) -> &'static str {
                match self {
                    #(#type_name_arms),*
                }
            }

            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    #(#from_str_arms),*,
                    _ => None,
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

/// Extract the string value from a `#[name = "..."]` attribute if present
fn extract_name_attribute(variant: &Variant) -> Result<Option<String>, Error> {
    let name_attr = variant
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("name"));

    if let Some(attr) = name_attr {
        match &attr.meta {
            Meta::NameValue(MetaNameValue { value, .. }) => {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) = value
                {
                    Ok(Some(lit_str.value()))
                } else {
                    Err(Error::new_spanned(
                        value,
                        "name attribute must have a string literal value"
                    ))
                }
            }
            _ => Err(Error::new_spanned(
                attr,
                "name attribute must be in the form #[name = \"value\"]"
            )),
        }
    } else {
        Ok(None)
    }
}

/// Derive macro that generates bidirectional conversion methods between enum variants and their numeric values.
/// This automatically generates:
/// - `from_int<T: Into<usize>>(value: T) -> Option<Self>`: Converts a numeric value to the corresponding enum variant
/// - `to_int(&self) -> usize`: Converts the enum variant back to its numeric value
///
/// ### Examples
///
/// Basic usage:
///
/// ```
/// use slang_derive::NumericEnum;
///
/// #[derive(Debug, PartialEq, NumericEnum)]
/// enum OpCode {
///     Add = 1,
///     Subtract = 2,
///     Multiply, // Implicit value: 3
///     Divide,   // Implicit value: 4
/// }
///
/// // Convert from numeric values to enum variants:
/// let add_op = OpCode::from_int(1u8); // Some(OpCode::Add)
/// let add_op_usize = OpCode::from_int(1usize); // Some(OpCode::Add)
/// let invalid_op = OpCode::from_int(100u8); // None
///
/// ```
///
/// Both explicit and implicit discriminant values are supported:
///
/// ```
/// use slang_derive::NumericEnum;
///
/// #[derive(NumericEnum)]
/// enum Status {
///     Ok = 200,
///     NotFound = 404,
///     ServerError = 500,
///     Created = 201,  // Note: order doesn't matter for explicit values
///     NotModified = 304,
///     BadRequest = 400,
/// }
/// ```
#[proc_macro_derive(NumericEnum)]
pub fn derive_numeric_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let variants = if let Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        return Error::new_spanned(
            &input.ident,
            "NumericEnum can only be derived for enums"
        ).to_compile_error().into();
    };

    let mut next_discriminant = 0usize;

    let mut variant_values = Vec::new();

    for variant in variants.iter() {
        let variant_name = &variant.ident;

        let value = if let Some((_, expr)) = &variant.discriminant {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(lit_int),
                ..
            }) = expr
            {
                match lit_int.base10_parse::<usize>() {
                    Ok(parsed_value) => {
                        next_discriminant = parsed_value + 1;
                        parsed_value
                    }
                    Err(_) => {
                        return Error::new_spanned(
                            lit_int,
                            "Enum discriminant must be a valid integer"
                        ).to_compile_error().into();
                    }
                }
            } else {
                return Error::new_spanned(
                    expr,
                    "NumericEnum requires integer literals as enum discriminants"
                ).to_compile_error().into();
            }
        } else {
            let value = next_discriminant;
            next_discriminant += 1;
            value
        };

        variant_values.push((variant_name, value));
    }

    let from_int_arms = variant_values.iter().map(|(variant_name, value)| {
        quote! {
            #value => Some(#enum_name::#variant_name)
        }
    });

    let expanded = quote! {
        impl #enum_name {
            pub fn from_int<T: Into<usize>>(value: T) -> Option<Self> {
                let value = value.into();
                match value {
                    #(#from_int_arms),*,
                    _ => None,
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

/// Derive macro that generates an iterator over all enum variants.
/// This creates a method `iter()` that returns an iterator over all possible values of the enum.
///
/// ### Example
/// ```
/// use slang_derive::IterableEnum;
///
/// #[derive(Debug, Copy, Clone, IterableEnum)]
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
///
/// // Iterate over all enum values
/// for color in Color::iter() {
///     println!("{:?}", color);
/// }
/// ```
///
/// This macro only works with enums that have no associated data (unit variants only).
#[proc_macro_derive(IterableEnum)]
pub fn derive_iterable_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let variants = if let Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        return Error::new_spanned(
            &input.ident,
            "IterableEnum can only be derived for enums"
        ).to_compile_error().into();
    };

    for variant in variants.iter() {
        if !variant.fields.is_empty() {
            return Error::new_spanned(
                variant,
                "IterableEnum can only be derived for enums with unit variants (no associated data)"
            ).to_compile_error().into();
        }
    }

    let variant_names = variants.iter().map(|variant| &variant.ident);
    let variant_count = variants.len();

    let expanded = quote! {
        impl #enum_name {
            pub fn iter() -> impl Iterator<Item = #enum_name> + Clone {
                const VARIANTS: [#enum_name; #variant_count] = [
                    #(#enum_name::#variant_names),*
                ];
                VARIANTS.iter().copied()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
