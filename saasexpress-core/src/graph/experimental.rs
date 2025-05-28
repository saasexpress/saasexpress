use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Debug)]
pub struct Origin<T> {
    pub inner: Arc<T>,
}

impl<T> Origin<T> {
    pub fn new(inner: T) -> Self {
        Origin {
            inner: Arc::new(inner),
        }
    }
}
impl<T> Clone for Origin<T> {
    fn clone(&self) -> Self {
        Origin {
            inner: Arc::clone(&self.inner),
        }
    }
}
impl<T> DerefMut for Origin<T> {
    fn deref_mut(&mut self) -> &mut T {
        Arc::get_mut(&mut self.inner).unwrap()
    }
}
impl<T> Deref for Origin<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> From<Arc<T>> for Origin<T> {
    fn from(inner: Arc<T>) -> Self {
        Origin { inner }
    }
}
impl<T> From<Origin<T>> for Arc<T> {
    fn from(origin: Origin<T>) -> Self {
        origin.inner
    }
}

#[derive(Debug)]
pub struct EdgeDestination {
    pub to: String,
    pub role: String,
}
