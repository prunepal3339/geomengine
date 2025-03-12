use num::{Num, Float};
use std::fmt::Debug;
use crate::point::Point2D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2D<T: Num + Copy + Debug> {
    pub p1: Point2D<T>,
    pub p2: Point2D<T>
}

impl<T: Num + Copy + Debug> Line2D<T> {
    pub fn new(p1: Point2D<T>, p2: Point2D<T>) -> Self {
        Line2D { p1, p2}
    }
    pub fn midpoint(&self) -> Point2D<T> {
        let mid_x = ( self.p1.x + self.p2.x ) / (T::one() + T::one());
        let mid_y = (self.p1.y + self.p2.y) / (T::one() + T::one());
        Point2D::new(mid_x, mid_y)
    }
}

impl<T: Float + Debug> Line2D<T> {
    pub fn length(&self) -> T {
        let x_diff = self.p2.x - self.p1.x;
        let y_diff = self.p2.y - self.p1.y;
        (x_diff * x_diff + y_diff * y_diff).sqrt()
    }
    pub fn slope(&self) -> Option<T> {
        if self.p2.x == self.p1.x { None }
        else{
            Some((self.p2.y - self.p1.y) / (self.p2.x - self.p1.x))
        }
    }
    pub fn subdivide(&self, num_segments: usize) -> Vec<Line2D<T>> {
        let mut subdivisions = Vec::new();
        
        if num_segments == 0 { 
            return subdivisions; 
        }

        let dx = (self.p2.x - self.p1.x) / T::from(num_segments).unwrap();
        let dy = (self.p2.y - self.p1.y) / T::from(num_segments).unwrap();

        for i in 0..num_segments {
            let start = Point2D::new(
                self.p1.x + T::from(i).unwrap() * dx,
                self.p1.y + T::from(i).unwrap() * dy,
            );
            let end = Point2D::new(
                self.p1.x + T::from(i + 1).unwrap() * dx,
                self.p1.y + T::from(i + 1).unwrap() * dy,
            );

            subdivisions.push(Line2D::new(start, end));
        }
        subdivisions
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_line2d_new() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(1.0, 1.0);
        let line = Line2D::new(p1, p2);

        assert_eq!(line.p1, p1);
        assert_eq!(line.p2, p2);
    }
    #[test]
    fn test_line2d_midpoint() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(2.0, 2.0);
        let line = Line2D::new(p1, p2);

        assert_eq!(line.midpoint(), Point2D::new(1.0, 1.0));
    }

    #[test]
    fn test_line2d_length() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(3.0, 4.0);
        let line = Line2D::new(p1, p2);

        assert_eq!(line.length(), 5.0);
    }

    #[test]
    fn test_line2d_slope_non_vertical_line() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(2.0, 2.0);
        let line = Line2D::new(p1, p2);
        
        assert_eq!(line.slope(), Some(1.0));
    }
    
    #[test]
    fn test_line2d_slope_vertical_line() {
        let p1 = Point2D::new(1.0, 3.0);
        let p2 = Point2D::new(1.0, 2.0);
        let vertical_line = Line2D::new(p1, p2);
        
        assert!(vertical_line.slope().is_none(), "Expected None: Cannot find slope of vertical line");
    }

    mod subdivisions_tests {
        use super::super::*;
        #[test]
        fn zero_segments_return_empty() {
            let line = Line2D::new(Point2D::new(0.0, 0.0), Point2D::new(1.0, 2.0));
            let result = line.subdivide(0);
            assert!(result.is_empty(), "Expect: empty vector");
        }

        #[test]
        fn one_segment_return_self() {
            let line = Line2D::new(Point2D::new(0.0, 0.0), Point2D::new(1.0, 1.0));
            let new_lines = line.subdivide(1);
            
            assert_eq!(new_lines.len(), 1);
            assert_eq!(new_lines[0], line);
        }

        #[test]
        fn two_segments_return_two_lines() {
            let line = Line2D::new(Point2D::new(0.0, 0.0), Point2D::new(1.2345, 1.2345));
            let divided_lines = line.subdivide(2);

            assert!(divided_lines.len() == 2, "Expect: length must equal the subdivision");

            //match start
            assert_eq!(divided_lines[0].p1.x, line.p1.x);
            assert_eq!(divided_lines[0].p1.y, line.p1.y);

            //match segment points
            assert_eq!(divided_lines[0].p2.x, divided_lines[1].p1.x);
            assert_eq!(divided_lines[0].p2.y, divided_lines[1].p1.y);


            //match value
            assert_eq!(divided_lines[0].p2.x, 0.61725);
            assert_eq!(divided_lines[0].p2.y, 0.61725);

            //match end
            assert_eq!(divided_lines[1].p2.x, line.p2.x);
            assert_eq!(divided_lines[1].p2.y, line.p2.y);
        }
    }
}