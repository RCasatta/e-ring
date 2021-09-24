//! hist
//!
//! This module provides implementation to draw histograms on a Display
//!

use crate::{FindRange, Range, Ring};
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::prelude::{DrawTarget, PixelColor, Primitive};
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics::Drawable;

/// Represent a histogram with values contained in the `ring` but rescaled to fit in the window
/// defined by the `upper_left` and `lower_right` points
#[derive(Debug)]
pub struct Hist {
    upper_left: Point,
    size: Size,
}

/// A struct containing three points
pub type ThreePoints = [Point; 3];

/// Errors in creating the histogram
#[derive(Debug)]
pub enum Error {
    /// The ring size must match the width (distance over the x axis) of the given points
    RingSizeMismatch {
        /// The hist window width
        width: u32,
        /// The data size
        ring_size: usize,
    },
    /// Draw error
    DrawError,
}

impl Hist {
    /// Create an Hist, checking if parameters are valid
    pub fn new(upper_left: Point, size: Size) -> Hist {
        Hist { upper_left, size }
    }

    /// The hist window size
    pub fn size(&self) -> &Size {
        &self.size
    }

    /// Draw the histogram on a display
    pub fn draw<C: PixelColor, D: DrawTarget<Color = C>, const N: usize>(
        &self,
        ring: &Ring<i16, N>,
        display: &mut D,
        foreground: C,
        background: C,
    ) -> Result<(), Error> {
        let lines = self.draw_lines(ring)?;
        for points in lines.iter() {
            Line::new(points[0], points[1])
                .into_styled(PrimitiveStyle::with_stroke(foreground, 1))
                .draw(display)
                .map_err(|_| Error::DrawError)?;
            Line::new(points[1], points[2])
                .into_styled(PrimitiveStyle::with_stroke(background, 1))
                .draw(display)
                .map_err(|_| Error::DrawError)?;
        }
        Ok(())
    }

    /// internal testable method, returning N tuples of 3 points (A,B,C)
    /// A->B will be foreground colored while B-C will be background colored
    fn draw_lines<const N: usize>(&self, ring: &Ring<i16, N>) -> Result<[ThreePoints; N], Error> {
        if ring.size() as u32 != self.size.width {
            return Err(Error::RingSizeMismatch {
                width: self.size.width,
                ring_size: ring.size(),
            });
        }
        let mut result = [ThreePoints::default(); N];
        let total_elements = ring.len();
        if total_elements > 0 {
            let range = ring.range().unwrap();
            let desired_range = Range::new(1i16, self.size.height as i16).unwrap();
            let baseline = self.upper_left.y + self.size.height as i32;
            for (i, resc) in ring.rescaled_iter(range, desired_range).enumerate() {
                let x = (self.upper_left.x as usize + self.size.width as usize - total_elements + i)
                    as i32;
                let a = Point::new(x, baseline);
                let b = Point::new(x, baseline - (resc as i32));
                let c = Point::new(x, baseline - self.size.height as i32 + 1);
                result[i] = [a, b, c];
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::{Error, Hist};
    use crate::hist::ThreePoints;
    use crate::Ring;
    use assert_matches::assert_matches;
    use embedded_graphics::geometry::{Point, Size};

    #[test]
    fn test_hist_draw() {
        let ring: Ring<i16, 2> = Ring::new();
        let z = Point::zero();

        let hist = Hist::new(z, Size::new(1, 1));
        let err = hist.draw_lines(&ring);
        assert_matches!(
            err,
            Err(Error::RingSizeMismatch {
                width: 1,
                ring_size: 2
            })
        );

        let hist = Hist::new(z, Size::new(2, 1));
        assert_matches!(hist.draw_lines(&ring), Ok(_));
    }

    #[test]
    fn test_hist() {
        let mut ring: Ring<i16, 3> = Ring::new();
        ring.append(1);
        ring.append(2);
        ring.append(3);
        let a = Point::zero();
        let b = Size::new(3, 5);
        let hist = Hist::new(a, b);
        assert_eq!(hist.size().height, 5);
        assert_eq!(hist.size().width, 3);
        let points = hist.draw_lines(&ring).unwrap();
        for t in points.iter() {
            // ensure no points is out of the rectangle [a,b]
            for p in t {
                assert!((p.x - a.x) <= b.width as i32);
                assert!((p.y - a.y) <= b.height as i32);
            }
        }

        let hist_string = hist_to_string(&hist, &ring);
        let expected = r#"
  #
  #
 ##
 ##
###
"#;
        assert_eq!(expected, hist_string);
    }

    /// utility to render a line of the hist at `height`
    /// `height=0` means the bottom pixel line of the hist
    fn line<const N: usize>(height: i16, points: &[ThreePoints; N]) -> [bool; N] {
        let mut result = [false; N];
        for (i, t) in points.iter().enumerate() {
            if t[1].y + (height as i32) < t[0].y {
                result[i] = true;
            }
        }
        result
    }

    fn hist_to_string<const N: usize>(hist: &Hist, ring: &Ring<i16, N>) -> String {
        let points = hist.draw_lines(&ring).unwrap();
        let height = hist.size().height;
        let mut result = String::new();
        result.push('\n');
        for h in (0..height as i16).rev() {
            let line_bool = line(h, &points);
            let line: String = line_bool
                .iter()
                .map(|e| if *e { "#" } else { " " })
                .collect();
            result.push_str(&line);
            result.push('\n');
        }
        result
    }
}
