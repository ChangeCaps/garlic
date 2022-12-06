use std::ops::Deref;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Order {
    indices: Vec<usize>,
}

impl Order {
    #[inline]
    pub const fn new() -> Self {
        Self {
            indices: Vec::new(),
        }
    }

    #[inline]
    pub fn resize(&mut self, size: usize) {
        if self.indices.len() == size {
            return;
        }

        if size < self.indices.len() {
            self.indices.truncate(size);
            return;
        }

        for i in self.indices.len()..size {
            self.indices.push(i);
        }
    }

    #[inline]
    pub fn swap_move(&mut self, from: usize, mut to: usize) {
        if from == to {
            return;
        }

        if from < to {
            to -= 1;
        }

        let index = self.indices.remove(from);
        self.indices.insert(to, index);
    }

    #[inline]
    #[track_caller]
    pub fn apply<T>(mut self, items: &mut [T]) {
        assert_eq!(
            items.len(),
            self.indices.len(),
            "items and indices must be the same length"
        );

        for i in 0..items.len() {
            while self.indices[i] != i {
                items.swap(i, self.indices[i]);

                let index = self.indices[i];
                self.indices.swap(i, index);
            }
        }
    }
}

impl Deref for Order {
    type Target = [usize];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.indices
    }
}
