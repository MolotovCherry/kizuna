#[doc(hidden)]
#[macro_export]
macro_rules! tri {
    ($($code:tt)*) => {{
        #[allow(clippy::redundant_closure_call)]
        (|| {
            $(
                $code
            )*
        })()
    }};
}

/// Poor mans try {} block
pub use tri;
