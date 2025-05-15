use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Lit, Meta, MetaNameValue, Expr, ExprLit};

/// Derive macro that generates `type_name()` and `from_str()` methods for enums
/// based on `#[type_name = "..."]` attributes on the variants.
#[proc_macro_derive(NamedEnum, attributes(name))]
pub fn derive_type_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let variants = if let Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        panic!("NamedEnum can only be derived for enums");
    };
    
    // Generate match arms for the type_name method
    let type_name_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        
        // Look for #[type_name = "..."] attribute
        let type_name = variant.attrs.iter()
            .find(|attr| attr.path().is_ident("name"))
            .map(|attr| {
                // Get the string value from the attribute
                let string_value = match &attr.meta {
                    Meta::NameValue(MetaNameValue { value, .. }) => {
                        if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = value {
                            lit_str.value()
                        } else {
                            panic!("name attribute must have a string literal value");
                        }
                    },
                    _ => panic!("name attribute must be in the form #[type_name = \"value\"]"),
                };
                string_value
            })
            .unwrap_or_else(|| variant_name.to_string().to_lowercase());
        
        quote! {
            #name::#variant_name => #type_name
        }
    });
    
    // Generate match arms for the from_str method
    let from_str_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        
        // Look for #[type_name = "..."] attribute
        let type_name = variant.attrs.iter()
            .find(|attr| attr.path().is_ident("name"))
            .map(|attr| {
                // Get the string value from the attribute
                let string_value = match &attr.meta {
                    Meta::NameValue(MetaNameValue { value, .. }) => {
                        if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = value {
                            lit_str.value()
                        } else {
                            panic!("name attribute must have a string literal value");
                        }
                    },
                    _ => panic!("name attribute must be in the form #[type_name = \"value\"]"),
                };
                string_value
            })
            .unwrap_or_else(|| variant_name.to_string().to_lowercase());
        
        quote! {
            #type_name => Some(#name::#variant_name)
        }
    });
    
    // Generate the implementation
    let expanded = quote! {
        impl #name {
            /// Get the string name of this type
            pub const fn name(&self) -> &'static str {
                match self {
                    #(#type_name_arms),*
                }
            }
            
            /// Try to create a PrimitiveType from a type name string
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