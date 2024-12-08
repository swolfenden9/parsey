use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, Data, DataEnum, DeriveInput, Ident, Meta};

pub fn ast_impl(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).expect("Token can only be derived for enums");

    let name = &input.ident;

    // Extract the #[ast] attribute
    let ast_attr = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("ast"))
        .expect("missing #[ast] attribute");
    let (token_type, errror_type) = parse_ast_attribute(ast_attr); // Parse the attribute into metadata

    // Generate the Ast implementation based on the input
    let expanded = match &input.data {
        Data::Enum(data_enum) => implement_ast_for_enum(name, data_enum, &token_type, &errror_type),
        _ => panic!("#[derive(Ast)] is only supported for enums."),
    };

    TokenStream::from(expanded)
}

fn parse_ast_attribute(attr: &Attribute) -> (Ident, Ident) {
    match &attr.meta {
        Meta::List(list) => {
            let token_type;
            let error_type;
            let mut tokens = list.tokens.clone().into_iter();

            if let Some(t) = tokens.next() {
                token_type = syn::parse2(t.into_token_stream()).expect("incorrect #[ast] format");
            } else {
                panic!("incorrect #[ast] format: expected Token type")
            }

            if let Some(_) = tokens.next() {
            } else {
                panic!("incorrect #[ast] format: expected Error type")
            }

            if let Some(e) = tokens.next() {
                error_type = syn::parse2(e.into_token_stream()).expect("incorrect #[ast] format");
            } else {
                panic!("incorrect #[ast] format: expected Error type")
            }

            if let Some(_) = tokens.next() {
                panic!("incorrect #[ast] format: should only be two types: Token and Error")
            }

            return (token_type, error_type);
        }
        _ => panic!("incorrect #[ast] format"),
    }
}

fn parse_ast_attributes(attr: &Attribute) -> Vec<Ident> {
    match &attr.meta {
        Meta::List(_list) => {
            let idents = vec![];
            idents
        }
        _ => panic!("incorrect #[ast] format"),
    }
}

fn implement_ast_for_enum(
    name: &Ident,
    data_enum: &DataEnum,
    token_type: &Ident,
    error_type: &Ident,
) -> TokenStream {
    let match_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let ast_attr = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("ast"))
            .expect(&format!(
                "variant {} missing #[ast] attribute",
                variant_name
            ));

        let token_patterns = parse_ast_attributes(ast_attr);

        if token_patterns.is_empty() {
            panic!("#[ast] attribute must not be empty");
        } else {
            // Generate token match patterns for each token in the list
            let pattern = quote! {
                (#(#token_patterns),*)
            };
            quote! {
                #pattern => Ok(#name::#variant_name),
            }
        }
    });

    // Generate the Ast implementation
    quote! {
        impl<#token_type, #error_type> Ast<#token_type, #error_type> for #name {
            fn parse<P>(parser: &mut std::iter::Peekable<P>) -> Result<Self, #error_type>
            where
                P: Parser<#token_type, #error_type>,
            {
                match parser.next() {
                    #(#match_arms)*
                    _ => Err(#error_type),
                }
            }
        }
    }
}
