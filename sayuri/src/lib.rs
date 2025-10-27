pub mod sync;
pub mod time;
mod tri;

pub mod macros {
    pub use super::tri::tri;
    pub use sayuri_proc::stringify_raw;
}
