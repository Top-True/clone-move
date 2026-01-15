use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, Token};
use syn::parse::{Parse, ParseStream, Result};
use quote::quote;

struct CloneMoveMacro {
    vars: Vec<Ident>,
    args: syn::ExprClosure,
}

impl Parse for CloneMoveMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        // 解析 `[v1, v2]`
        let content;
        syn::bracketed!(content in input);
        let vars: Vec<Ident> = content.parse_terminated(Ident::parse, Token![,])?
            .into_iter()
            .collect();

        // 解析闭包表达式
        let args = input.parse()?;

        Ok(CloneMoveMacro { vars, args })
    }
}

#[proc_macro]
pub fn clone_move(input: TokenStream) -> TokenStream {
    let CloneMoveMacro { vars, args } = parse_macro_input!(input as CloneMoveMacro);

    // 提取闭包的参数和主体
    let closure_inputs = &args.inputs;
    let closure_body = &args.body;

    // 生成代码
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
