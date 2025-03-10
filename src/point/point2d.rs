use num::{Num, Float};
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Debug, Display, self};

#[derive(Debug, Clone, Copy)]
pub struct Point2D<T: Num + Copy + Debug> {
    x: T,
    y: T,
}

impl<T: Num + Copy + Debug> Point2D<T> {

    pub fn new(x: T, y: T) -> Self {
        Point2D{x , y}
    }
    pub fn origin() -> Self where T: num::Zero {
        Point2D{
            x: T::zero(),
            y: T::zero(),
        }
    }
    pub fn translate(&self, param: (T, T)) -> Self {
        Point2D{
            x: self.x + param.0,
            y: self.y + param.1
        }
    }
    pub fn translate_x(&self, dx: T) -> Self {
        Point2D{
            x: self.x + dx,
            y: self.y
        }
    }
    pub fn translate_y(&self, dy: T) -> Self {
        Point2D{
            x: self.x,
            y: self.y + dy
        }
    }
}
impl<T: Num + Copy + Debug> Add for Point2D<T> {
    type Output = Self;
    fn add(self, other: Point2D<T>) -> Point2D<T> {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

}
impl<T: Num + Copy + Debug> Sub for Point2D<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Point2D{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T: Num + Copy + Debug> Mul<T> for Point2D<T> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        Point2D{
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: Num + Copy + Debug> Div<T> for Point2D<T> {
    type Output = Self;
    fn div(self, scalar: T) -> Self::Output {
        Point2D {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}
impl<T: Num + Copy + Debug + Display> Display for Point2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Float + Debug> Point2D<T> {
    pub fn distance(&self, other: &Point2D<T>) -> T {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)).sqrt()
    }
    pub fn dot_product(&self, other: &Point2D<T>) -> T {
        self.x * other.x + self.y * other.y
    }
    pub fn cross_product(&self, other: &Point2D<T>) -> T {
        self.x * other.y - self.y * other.x
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_point2d_new() {
        let point = Point2D::new(1.0, 2.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
    }

    #[test]
    fn test_point2d_origin() {
        let point: Point2D<f64> = Point2D::origin();
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
    }

    #[test]
    fn test_point2d_add() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = Point2D::new(3.0, 4.0);
        
        let result = p1 + p2;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }

    #[test]
    fn test_point2d_sub() {
        let p1 = Point2D::new(5.0, 7.0);
        let p2 = Point2D::new(2.0, 3.0);
        
        let result = p1 - p2;

        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 4.0);
    }

    #[test]
    fn test_point2d_mul() {
        let p = Point2D::new(2.0, 3.0);
        
        let result = p * 2.0;

        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }
    #[test]
    fn test_point2d_div() {
        let p = Point2D::new(4.0, -2.0);

        let result = p / 2.0;

        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, -1.0);
    }

    #[test]
    fn test_point2d_distance() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(3.0, 4.0);

        let distance = p1.distance(&p2);
        
        assert_eq!(distance, 5.0);
    }

    #[test]
    fn test_point2d_dot_product() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = Point2D::new(3.0, 4.0);

        let dot = p1.dot_product(&p2);

        assert_eq!(dot, 11.0);
    }

    #[test]
    fn test_point2d_cross_product() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = Point2D::new(3.0, 4.0);

        let cross = p1.cross_product(&p2);

        assert_eq!(cross, -2.0);
    }
}