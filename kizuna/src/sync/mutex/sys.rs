#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use unix::Mutex;
#[cfg(windows)]
pub use windows::Mutex;
