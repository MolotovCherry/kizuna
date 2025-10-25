mod stringify_raw;

use proc_macro::TokenStream;

/// Does stringify just like `stringify!`, but keeps
/// all whitespace/newlines intact.
///
/// Each tab is converted into one space.
#[proc_macro]
pub fn stringify_raw(input: TokenStream) -> TokenStream {
    stringify_raw::stringify(input)
}
