#[doc(hidden)]
#[macro_export]
macro_rules! _internal_tri {
    ($($code:tt)*) => {{
        const fn once<F, Output>(f: F) -> F
            where F: FnOnce() -> Output
        {
            f
        }

        once(|| { $($code)* })()
    }};
}

/// Poor man's try {} block
pub use _internal_tri as tri;
