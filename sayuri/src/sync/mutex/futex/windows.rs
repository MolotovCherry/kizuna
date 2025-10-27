use std::ffi::c_void;
use std::ptr;
use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr, AtomicU8,
    AtomicU16, AtomicU32, AtomicU64, AtomicUsize,
};
use std::time::Duration;

use windows::Win32::Foundation::{ERROR_TIMEOUT, GetLastError};
use windows::Win32::System::Threading::{INFINITE, WaitOnAddress, WakeByAddressSingle};

pub fn dur2timeout(dur: Duration) -> u32 {
    // Note that a duration is a (u64, u32) (seconds, nanoseconds) pair, and the
    // timeouts in windows APIs are typically u32 milliseconds. To translate, we
    // have two pieces to take care of:
    //
    // * Nanosecond precision is rounded up
    // * Greater than u32::MAX milliseconds (50 days) is rounded up to INFINITE
    //   (never time out).
    dur.as_secs()
        .checked_mul(1000)
        .and_then(|ms| ms.checked_add((dur.subsec_nanos() as u64) / 1_000_000))
        .and_then(|ms| {
            ms.checked_add(if !dur.subsec_nanos().is_multiple_of(1_000_000) {
                1
            } else {
                0
            })
        })
        .map(|ms| {
            if ms > <u32>::MAX as u64 {
                INFINITE
            } else {
                ms as u32
            }
        })
        .unwrap_or(INFINITE)
}

/// An atomic for use as a futex that is at least 8-bits but may be larger.
pub type SmallFutex = AtomicU8;
/// Must be the underlying type of SmallFutex
pub type SmallPrimitive = u8;

#[allow(clippy::missing_safety_doc)]
pub unsafe trait Futexable {}
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Waitable {
    type Futex;
}
macro_rules! unsafe_waitable_int {
    ($(($int:ty, $atomic:ty)),*$(,)?) => {
        $(
            unsafe impl Waitable for $int {
                type Futex = $atomic;
            }
            unsafe impl Futexable for $atomic {}
        )*
    };
}
unsafe_waitable_int! {
    (bool, AtomicBool),
    (i8, AtomicI8),
    (i16, AtomicI16),
    (i32, AtomicI32),
    (i64, AtomicI64),
    (isize, AtomicIsize),
    (u8, AtomicU8),
    (u16, AtomicU16),
    (u32, AtomicU32),
    (u64, AtomicU64),
    (usize, AtomicUsize),
}
unsafe impl<T> Waitable for *const T {
    type Futex = AtomicPtr<T>;
}
unsafe impl<T> Waitable for *mut T {
    type Futex = AtomicPtr<T>;
}
unsafe impl<T> Futexable for AtomicPtr<T> {}

pub fn wait_on_address<W: Waitable>(
    address: &W::Futex,
    compare: W,
    timeout: Option<Duration>,
) -> bool {
    unsafe {
        let addr = ptr::from_ref(address).cast::<c_void>();
        let size = size_of::<W>();
        let compare_addr = (&raw const compare).cast::<c_void>();
        let timeout = timeout.map(dur2timeout).unwrap_or(INFINITE);
        WaitOnAddress(addr, compare_addr, size, Some(timeout)).is_ok()
    }
}

pub fn wake_by_address_single<T: Futexable>(address: &T) {
    unsafe {
        let addr = ptr::from_ref(address).cast::<c_void>();
        WakeByAddressSingle(addr);
    }
}

pub fn futex_wait<W: Waitable>(futex: &W::Futex, expected: W, timeout: Option<Duration>) -> bool {
    // return false only on timeout
    wait_on_address(futex, expected, timeout) || unsafe { GetLastError() } != ERROR_TIMEOUT
}

pub fn futex_wake<T: Futexable>(futex: &T) -> bool {
    wake_by_address_single(futex);
    false
}
