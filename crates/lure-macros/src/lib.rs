use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};

#[proc_macro]
pub fn expand_regex(input: TokenStream) -> TokenStream {
    let re = input.to_string();
    match regex::Regex::new(&re) {
        Ok(_) => input,
        Err(e) => generate_compile_error(format!("Invalid regex: {}", e)),
    }
}

fn generate_compile_error(e: impl ToString) -> TokenStream {
    let msg = e.to_string(); // Convert error message to string

    let tokens = vec![
        // "::"
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        // "std"
        TokenTree::Ident(Ident::new("std", proc_macro::Span::call_site())),
        // "::"
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        // "compile_error"
        TokenTree::Ident(Ident::new("compile_error", proc_macro::Span::call_site())),
        // "!"
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        // "()" with a string literal inside
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            vec![TokenTree::Literal(Literal::string(&msg))]
                .into_iter()
                .collect(),
        )),
    ];

    tokens.into_iter().collect()
}
