use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, ExprLit, Lit, Meta, MetaNameValue, Variant, parse_macro_input};

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
pub fn derive_type_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let variants = if let Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        panic!("NamedEnum can only be derived for enums");
    };
    let variant_mappings = variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let string_name = extract_name_attribute(variant)
                .unwrap_or_else(|| variant_name.to_string().to_lowercase());
            (variant_name, string_name)
        })
        .collect::<Vec<_>>();

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
fn extract_name_attribute(variant: &Variant) -> Option<String> {
    variant
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("name"))
        .map(|attr| match &attr.meta {
            Meta::NameValue(MetaNameValue { value, .. }) => {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) = value
                {
                    lit_str.value()
                } else {
                    panic!("name attribute must have a string literal value");
                }
            }
            _ => panic!("name attribute must be in the form #[name = \"value\"]"),
        })
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
        panic!("NumericEnum can only be derived for enums");
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
                let parsed_value = lit_int
                    .base10_parse::<usize>()
                    .expect("Enum discriminant must be a valid integer");
                next_discriminant = parsed_value + 1;
                parsed_value
            } else {
                panic!("NumericEnum requires integer literals as enum discriminants");
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
