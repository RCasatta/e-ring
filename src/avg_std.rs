use crate::Ring;
use core::ops::{Add, Div, Mul, Sub};

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
    /// Calculate the average of the elements in the `Ring`
    pub fn avg(&self) -> f32 {
        let mut acc = 0.0f32;
        for el in self.iter() {
            acc += el.into();
        }
        let len: f32 = (self.len() as u16).into(); //FIXME cast
        acc / len
    }

    /// Calculate the variance of the elements in the `Ring`, use provided `avg` if `Some`,
    /// otherwise it calculates it (in the latter case two iterations are required).
    pub fn var(&self, avg: Option<f32>) -> f32 {
        let avg = avg.unwrap_or_else(|| self.avg());
        let mut acc = 0.0f32;
        for el in self.iter() {
            let val = el.into() - avg;
            acc += val * val;
        }
        let len: f32 = (self.len() as u16).into(); //FIXME cast
        acc / len
    }
}
