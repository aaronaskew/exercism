pub struct CircularBuffer<T: Clone> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    oldest: Option<usize>,
    newest: Option<usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity],
            capacity,
            oldest: None,
            newest: None,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }

        if self.is_empty() {
            self.oldest = Some(0);
            self.newest = Some(0);
        } else {
            self.newest = self.newest.map(|newest| (newest + 1) % self.capacity);
        }

        self.buffer[self.newest.unwrap()] = Some(element);

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }

        let res = self.buffer[self.oldest.unwrap()].clone().unwrap();
        self.buffer[self.oldest.unwrap()] = None;
        if self.oldest == self.newest {
            self.clear();
        } else {
            self.oldest = self.oldest.map(|oldest| (oldest + 1) % self.capacity);
        }

        Ok(res)
    }

    pub fn clear(&mut self) {
        *self = Self::new(self.capacity);
    }

    pub fn overwrite(&mut self, element: T) {
        if self.is_empty() {
            self.oldest = Some(0);
            self.newest = Some(0);
            self.buffer[self.oldest.unwrap()] = Some(element);
        } else if self.is_full() {
            self.buffer[self.oldest.unwrap()] = Some(element);
            self.newest = self.oldest;
            self.oldest = self.oldest.map(|oldest| (oldest + 1) % self.capacity);
        } else {
            self.newest = self.newest.map(|newest| (newest + 1) % self.capacity);
            self.buffer[self.newest.unwrap()] = Some(element);
        }
    }

    fn is_empty(&self) -> bool {
        self.oldest.is_none() && self.newest.is_none()
    }

    fn is_full(&self) -> bool {
        if let Some(oldest) = self.oldest
            && let Some(newest) = self.newest
            && (((oldest + self.capacity - newest) % self.capacity == 1)
                || (self.capacity == 1 && oldest == newest))
        {
            true
        } else {
            false
        }
    }
}
