use std::{
    fmt::Display,
    ops::Deref,
    sync::{atomic::Ordering, Arc},
};

use crate::{ref_inner::RefInner, ref_mut::RefMut};

pub struct Ref<T> {
    pub(crate) inner: Arc<RefInner<T>>,
}

unsafe impl<T> Sync for Ref<T> {}
unsafe impl<T> Send for Ref<T> {}

impl<T> Ref<T> {
    pub fn new(data: T) -> Self {
        Self {
            inner: Arc::new(RefInner::new(data)),
        }
    }

    pub fn locked(&self) -> bool {
        self.inner.lock.load(Ordering::Relaxed) > 0
    }

    /// # Dont use!!!
    pub fn lock(&self) {
        while self
            .inner
            .lock
            .compare_exchange_weak(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            atomic_wait::wait(&self.inner.lock, 1);
        }
    }

    /// # Dont use!!!
    pub fn unlock(&self) {
        self.inner.lock.store(0, Ordering::Release);
        atomic_wait::wake_one(&self.inner.lock)
    }

    /// Recomand to use `get_mut`
    /// this is not really recomanded
    /// but if you do things in a single thread is more recomanded
    pub fn mut_scope(&self, clasure: impl Fn(&mut T)) {
        self.lock();
        clasure(unsafe { &mut *self.inner.cell.get() });
        self.unlock()
    }

    /// if you doing things in as single thread if you call get_mut 2 times or more the application will be in a loop
    /// Dont `get_mut` more then one time!
    pub fn get_mut(&self) -> RefMut<T> {
        self.lock();

        RefMut {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.cell.get() }
    }
}

impl<T> Display for Ref<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::Ref;
    use std::thread::spawn;

    #[test]
    fn threading_1() {
        let data = Ref::new(0);

        let tmp_data = data.clone();
        let thread1 = spawn(move || {
            let data = tmp_data;
            for _ in 0..5000000 {
                let mut data = data.get_mut();
                *data += 1;
            }
        });

        let tmp_data = data.clone();
        let thread2 = spawn(move || {
            let data = tmp_data;
            for _ in 0..5000000 {
                let mut data = data.get_mut();
                *data += 1;
            }
        });

        let tmp_data = data.clone();
        let thread3 = spawn(move || {
            let data = tmp_data;
            for _ in 0..5000000 {
                let mut data = data.get_mut();
                *data += 1;
            }
        });

        thread1.join().unwrap();
        thread2.join().unwrap();
        thread3.join().unwrap();

        assert_eq!(*data, 15000000)
    }
}
