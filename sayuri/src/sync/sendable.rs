/// Make any T Send+Sync
pub struct Sendable<T: ?Sized>(T);

unsafe impl<T> Send for Sendable<T> {}
unsafe impl<T> Sync for Sendable<T> {}

impl<T> Sendable<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }

    /// # Safety
    /// All safety requirements for `T` apply
    pub unsafe fn as_ref(&self) -> &T {
        &self.0
    }

    /// # Safety
    /// All safety requirements for `T` apply
    pub unsafe fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }

    /// # Safety
    /// All safety requirements for `T` apply
    pub unsafe fn into_inner(self) -> T {
        self.0
    }
}
