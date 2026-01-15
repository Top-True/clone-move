use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, Token, parse_macro_input};

struct CloneMoveMacro {
    vars: Vec<Ident>,
    args: syn::ExprClosure,
}

impl Parse for CloneMoveMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        syn::bracketed!(content in input);
        let vars: Vec<Ident> = content
            .parse_terminated(Ident::parse, Token![,])?
            .into_iter()
            .collect();
        let args = input.parse()?;
        Ok(CloneMoveMacro { vars, args })
    }
}

#[proc_macro]
pub fn clone_move(input: TokenStream) -> TokenStream {
    let CloneMoveMacro { vars, args } = parse_macro_input!(input as CloneMoveMacro);
    let closure_inputs = &args.inputs;
    let closure_body = &args.body;
    let expanded = quote! {
        {
            #(
                let #vars = ::core::clone::Clone::clone(&#vars);
            )*
            move |#closure_inputs| #closure_body
        }
    };

    TokenStream::from(expanded)
}

struct AsyncMoveMacro {
    vars: Vec<Ident>,
    args: syn::Block,
}

impl Parse for AsyncMoveMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        syn::bracketed!(content in input);
        let vars: Vec<Ident> = content
            .parse_terminated(Ident::parse, Token![,])?
            .into_iter()
            .collect();
        let args = input.parse()?;
        Ok(AsyncMoveMacro { vars, args })
    }
}

#[proc_macro]
pub fn async_move(input: TokenStream) -> TokenStream {
    let AsyncMoveMacro { vars, args } = parse_macro_input!(input as AsyncMoveMacro);
    let expanded = quote! {
        {
            #(
                let #vars = ::core::clone::Clone::clone(&#vars);
            )*
            async move #args
        }
    };
    TokenStream::from(expanded)
}
