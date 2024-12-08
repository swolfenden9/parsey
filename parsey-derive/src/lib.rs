use proc_macro::TokenStream;

mod ast;

#[proc_macro_derive(Ast, attributes(ast))]
pub fn derive_ast(input: TokenStream) -> TokenStream {
    ast::ast_impl(input.into()).into()
}
