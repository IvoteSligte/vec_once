use std::ops::{Deref, DerefMut};

/// Iterates over a `Vec`, returning each element exactly once. Allows modification of the underlying data. That's it.
#[derive(Clone)]
pub struct VecOnce<T: Clone> {
    index: usize,
    inner: Vec<T>,
}

impl<T: Clone> Iterator for VecOnce<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.inner.get(self.index).map(|x| x.clone());
        self.index += 1;

        value
    }
}

impl<T: Clone> VecOnce<T> {
    /// Creates a `VecOnce` from a `Vec`
    pub fn new(vector: Vec<T>) -> Self {
        Self::from(vector)
    }

    /// Returns the index. Panics if the vector is empty.
    pub fn index(&self) -> usize {
        assert!(self.inner.len() != 0, "The contained vector is empty!");
        self.index
    }

    /// Sets the index. Panics if the index is out of range.
    pub fn set_index(&mut self, new_index: usize) {
        assert!(
            new_index < self.inner.len(),
            "The given index is too large!"
        );
        self.index = new_index;
    }

    /// Resets the index, making the iteration restart from index zero.
    pub fn restart(&mut self) {
        self.index = 0;
    }
}

impl<T: Clone> Deref for VecOnce<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Clone> DerefMut for VecOnce<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Clone> From<Vec<T>> for VecOnce<T> {
    fn from(vector: Vec<T>) -> Self {
        Self {
            index: 0,
            inner: vector,
        }
    }
}
