use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields, FieldsUnnamed};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[proc_macro_derive(EnumStr, attributes(enum2pos))]
pub fn derive_enum2pos(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let data = match input.data {
        Data::Enum(data) => data,
        _ => return derive_error!("enum2pos only supports enums"),
    };

    let mut match_arms = TokenStream2::new();

    for variant in data.variants.iter() {
        let variant_name = &variant.ident;

        match variant.fields {
            Fields::Unit => {
                let mut display_ident = "{}".to_string().to_token_stream();

                for attr in &variant.attrs {
                    if attr.path.is_ident("enum2pos") && attr.path.segments.first().is_some() {
                        match attr.parse_args::<syn::LitStr>() {
                            Ok(literal) => display_ident = literal.to_token_stream(),
                            Err(_) => {
                                return derive_error!(
                                    r#"The 'enum2pos' attribute is missing a String argument. Example: #[enum2pos("Listening on: {} {}")] "#
                                );
                            }
                        }
                    }
                }

                match_arms.extend(quote_spanned! {
                    variant.span() =>
                        #name::#variant_name =>  write!(f, "{}", stringify!(#display_ident)),
                });
            }
            Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => {
                let mut format_ident = "{}".to_string().to_token_stream();

                for attr in &variant.attrs {
                    if attr.path.is_ident("enum2pos") && attr.path.segments.first().is_some() {
                        match attr.parse_args::<syn::LitStr>() {
                            Ok(literal) => format_ident = literal.to_token_stream(),
                            Err(_) => {
                                return derive_error!(
                                    r#"The 'enum2pos' attribute is missing a String argument. Example: #[enum2pos("Listening on: {} {}")] "#
                                );
                            }
                        }
                    }
                }

                let fields = unnamed.iter().len();
                let args = ('a'..='z')
                    .take(fields)
                    .map(|letter| Ident::new(&letter.to_string(), variant.span()))
                    .collect::<Vec<_>>();
                match_arms.extend(quote_spanned! {
                    variant.span() => #name::#variant_name(#(#args),*) =>  write!(f, #format_ident, #(#args),*),
                });
            }
            _ => {
                return derive_error!(
                    "enum2pos is only implemented for unit and unnamed-field enums"
                )
            }
        };
    }

    let expanded = quote! {
        impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self {
                    #match_arms
                }
            }
        }
    };

    TokenStream::from(expanded)
}