use proc_macro::{Literal, TokenStream, TokenTree};

pub fn stringify(input: TokenStream) -> TokenStream {
    let mut source = String::new();
    let mut line = None;

    let mut new_line = true;
    let mut last_col = 0;
    for tree in input {
        let span = tree.span();

        let line = line.get_or_insert(span.line());

        if *line < span.line() {
            let how_many = span.line() - *line;
            let s = "\n".repeat(how_many);
            source.push_str(&s);
            *line = span.line();
            last_col = 0;
            new_line = true;
        }

        if let Some(s) = span.source_text() {
            let how_many = if new_line {
                new_line = false;
                span.column().saturating_sub(1)
            } else {
                span.column() - last_col
            };

            let spaces = " ".repeat(how_many);
            source.push_str(&spaces);
            source.push_str(&s);
        }

        last_col = span.end().column();
    }

    // custom Literal ToTokens impl
    let lit = Literal::string(&source);
    let mut stream = TokenStream::new();
    // custom extend_one
    stream.extend(Some(TokenTree::from(lit)));

    stream
}
