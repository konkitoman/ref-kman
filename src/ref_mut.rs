use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    sync::{atomic::Ordering, Arc},
};

use crate::ref_inner::RefInner;

pub struct RefMut<T> {
    pub(crate) inner: Arc<RefInner<T>>,
}

unsafe impl<T> Sync for RefMut<T> {}
unsafe impl<T> Send for RefMut<T> {}

impl<T> Deref for RefMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.cell.get() }
    }
}

impl<T> DerefMut for RefMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner.cell.get() }
    }
}

impl<T> Drop for RefMut<T> {
    fn drop(&mut self) {
        self.inner.lock.store(0, Ordering::Release);
        atomic_wait::wake_one(&self.inner.lock);
    }
}

impl<T> Display for RefMut<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}
