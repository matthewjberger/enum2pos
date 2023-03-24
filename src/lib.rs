use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[proc_macro_derive(EnumIndex, attributes(enum2pos))]
pub fn derive_enum2pos(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let data = match input.data {
        Data::Enum(data) => data,
        _ => return derive_error!("enum2pos only supports enums"),
    };

    let mut from_index_arms = TokenStream2::new();
    let mut to_index_arms = TokenStream2::new();

    for (index, variant) in data.variants.iter().enumerate() {
        let variant_name = &variant.ident;

        match variant.fields {
            Fields::Unit => {
                to_index_arms.extend(quote_spanned! {
                    variant.span() =>
                        #name::#variant_name => #index,
                });

                from_index_arms.extend(quote_spanned!(
                    variant.span() =>
                        #index => Some(#name::#variant_name),
                ));
            }

            Fields::Unnamed(ref fields) => {
                let field_count = fields.unnamed.len();
                let field_pats = (0..field_count).map(|_| quote!(_)).collect::<Vec<_>>();

                to_index_arms.extend(quote_spanned! {
                    variant.span() =>
                        #name::#variant_name(#(#field_pats),*) => #index,
                });

                let field_types = fields.unnamed.iter().map(|f| &f.ty);
                let field_constructors = field_types.clone().map(
                    |ty| quote_spanned! { ty.span() => args_iter.next()?.parse::<#ty>().ok()? },
                );
                from_index_arms.extend(quote_spanned! {
                    variant.span() =>
                        #index => {
                            let mut args_iter = args.into_iter();
                            Some(#name::#variant_name(#(#field_constructors),*))
                        },
                });
            }

            Fields::Named(..) => {
                to_index_arms.extend(quote_spanned! {
                    variant.span() =>
                        #name::#variant_name{..} => #index,
                });
            }
        };
    }

    let expanded = quote! {
        impl #name {
            pub fn from_index(index: usize, mut args: Vec<String>) -> Option<Self> {
                match index {
                    #from_index_arms
                    _ => None
                }
            }

            pub fn to_index(&self) -> usize {
                match self {
                    #to_index_arms
                }
            }
        }
    };

    TokenStream::from(expanded)
}
