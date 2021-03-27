use crate::ring::RingIterator;
use crate::Ring;
use core::ops::{Add, Div, Mul, Sub};

/// Contains min and max value in a `Ring`
#[derive(Debug)]
pub struct Range<T> {
    /// Minimum value
    pub min: T,
    /// Maximum value
    pub max: T,
}

/// Trait defining a `range` method to find min and max in one iteration
pub trait FindRange<T> {
    /// calculate min and max with one iteration
    fn range(&self) -> Option<Range<T>>;
}

impl<T: PartialOrd + Copy + Default, const N: usize> FindRange<T> for Ring<T, N> {
    fn range(&self) -> Option<Range<T>> {
        if self.len() == 0 {
            return None;
        }
        let mut iter = self.iter();
        let first = iter.next().unwrap(); // safe because len just checked;
        let mut min_max = Range {
            min: first,
            max: first,
        };
        for el in iter {
            if min_max.min.gt(&el) {
                min_max.min = el;
            }
            if min_max.max.lt(&el) {
                min_max.max = el;
            }
        }

        Some(min_max)
    }
}

#[derive(Debug)]
pub struct RescaleIterator<'a, T, const N: usize> {
    current: Range<T>,
    desired: Range<T>,
    ring_iter: RingIterator<'a, T, N>,
}

impl<
        T: Copy
            + Default
            + PartialOrd
            + Sub<Output = T>
            + Add<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Into<f32>,
        const N: usize,
    > Ring<T, N>
{
    /// Returns an iterator over the `Ring` on which values are rescaled according to the `desired`
    /// range
    pub fn rescaled_iter(&self, current: Range<T>, desired: Range<T>) -> RescaleIterator<T, N> {
        RescaleIterator {
            current,
            desired,
            ring_iter: self.iter(),
        }
    }
}

impl<
        T: Copy
            + Default
            + PartialOrd
            + Sub<Output = T>
            + Add<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Into<f32>,
        const N: usize,
    > Iterator for RescaleIterator<'_, T, N>
{
    type Item = f32;
    // TODO would be nice if type returned is `T`

    fn next(&mut self) -> Option<Self::Item> {
        self.ring_iter.next().map(|el| {
            let mut zero_one =
                (el.into() - self.current.min.into()) / (self.current.delta().into());
            if zero_one.is_nan() {
                zero_one = 0.5;
            }
            zero_one * self.desired.delta().into() + self.desired.min.into()
        })
    }
}

impl<T: Sub<Output = T> + Copy> Range<T> {

    /// Returns the range delta
    pub fn delta(&self) -> T {
        self.max - self.min
    }
}

#[cfg(test)]
mod test {
    use super::{FindRange, Range, Ring};
    const RING_SIZE: usize = 128;

    #[test]
    pub fn test_range() {
        let mut circ: Ring<i32, RING_SIZE> = Ring::new();
        assert!(circ.range().is_none());
        circ.append(0);
        assert_eq!(circ.range().unwrap().min, 0);
        assert_eq!(circ.range().unwrap().max, 0);
        circ.append(1);
        assert_eq!(circ.range().unwrap().min, 0);
        assert_eq!(circ.range().unwrap().max, 1);
        circ.append(-1);
        assert_eq!(circ.range().unwrap().min, -1);
        assert_eq!(circ.range().unwrap().max, 1);
        for _ in 0..RING_SIZE {
            circ.append(0);
        }
        assert_eq!(circ.range().unwrap().min, 0);
        assert_eq!(circ.range().unwrap().max, 0);
    }

    #[test]
    pub fn test_rescale() {
        let mut circ: Ring<i16, RING_SIZE> = Ring::new();
        circ.append(100i16);
        circ.append(200);
        circ.append(300);
        let current = circ.range().unwrap();
        let desired = Range { min: 20, max: 30 };
        let mut rescaled = circ.rescaled_iter(current, desired);
        assert_eq!(rescaled.next().map(|el| el as i16), Some(20i16));
        assert_eq!(rescaled.next().map(|el| el as i16), Some(25i16));
        assert_eq!(rescaled.next().map(|el| el as i16), Some(30i16));
        assert_eq!(rescaled.next(), None);
    }
}
