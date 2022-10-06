use std::{cell::UnsafeCell, sync::atomic};

pub struct RefInner<T> {
    pub(crate) cell: UnsafeCell<T>,
    pub(crate) lock: atomic::AtomicU32,
}

impl<T> RefInner<T> {
    pub(crate) fn new(data: T) -> Self {
        let data = UnsafeCell::new(data);
        Self {
            cell: data,
            lock: atomic::AtomicU32::new(0),
        }
    }
}
