/// Append only data structure, replace oldest element when reach maximum capacity of `N` elements
#[derive(Debug, Clone)]
pub struct Ring<T, const N: usize> {
    data: [T; N],
    next: usize,
    len: usize,
}

/// Iterator over `Ring` starting from the oldest element
#[derive(Debug)]
pub struct RingIterator<'a, T, const N: usize> {
    start: usize,
    count: usize,
    circular: &'a Ring<T, N>,
}

impl<T: Copy + Default, const N: usize> Default for Ring<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Default, const N: usize> Ring<T, N> {
    /// Creates a new `Ring` of give size `N`
    pub fn new() -> Self {
        Ring {
            data: [T::default(); N],
            next: 0usize,
            len: 0usize,
        }
    }

    fn increment_next(&mut self) {
        self.next = (self.next + 1) % self.data.len()
    }

    /// Append an element to the `Ring`, if there are already `N` elements, it replaces the oldest.
    pub fn append(&mut self, el: T) {
        self.data[self.next] = el;
        self.len = self.data.len().min(self.len + 1);
        self.increment_next()
    }

    /// Number of elements in the `Ring`, it never decreases.
    pub fn len(&self) -> usize {
        self.len
    }

    /// If the `Ring` is empty. Zero elements
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Return the max size of the ring
    pub fn size(&self) -> usize {
        N
    }

    /// Return the last item inserted
    pub fn last(&self) -> Option<T> {
        if self.len == 0 {
            None
        } else if self.next == 0 {
            Some(self.data[self.data.len() - 1])
        } else {
            Some(self.data[self.next - 1])
        }
    }

    /// Returns an iterator over the `Ring` starting from the oldest appended element
    pub fn iter(&self) -> RingIterator<T, N> {
        RingIterator {
            circular: &self,
            start: if self.len() == self.data.len() {
                self.next
            } else {
                0
            },
            count: 0usize,
        }
    }
}

impl<'a, T: Copy + Default, const N: usize> Iterator for RingIterator<'a, T, N> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let len = self.circular.len();
        if self.count == len {
            return None;
        }
        let current_index = (self.start + self.count) % len;
        let result = self.circular.data[current_index];
        self.count += 1;
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::Ring;

    const RING_SIZE: usize = 256;

    #[test]
    pub fn test_ring() {
        let mut circ: Ring<u32, RING_SIZE> = Ring::new();
        assert_eq!(circ.last(), None);
        assert_eq!(0, circ.len());
        circ.append(1u32);
        assert_eq!(circ.last(), Some(1));
        assert_eq!(1, circ.len());
        circ.append(2);
        circ.append(3);

        let mut iter = circ.iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
        assert_eq!(3, circ.len());
        for i in 0..1000 {
            circ.append(i);
            assert_eq!(circ.last(), Some(i));
        }
        assert_eq!(RING_SIZE, circ.len());

        let mut iter = circ.iter();

        for i in (1000 - RING_SIZE as u32)..1000 {
            assert_eq!(iter.next(), Some(i));
        }
        assert_eq!(iter.next(), None);
    }
}
