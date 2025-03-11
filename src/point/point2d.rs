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

    
    /// Normalize the point: unit vector like conversion
    pub fn normalize(&self) -> Option<Point2D<T>> {
        let magnitude = (self.x * self.x + self.y * self.y).sqrt();
        
        // if it is a zero vector representation
        if magnitude == T::zero() {
            return None; // cannot normalize a zero vector
        }

        Some( Point2D {
            x: self.x / magnitude,
            y: self.y / magnitude,
        })
    }
     /// rotates the point around another point (defaulting to origin)
     pub fn rotate(&self, angle: f64, center: Option<Point2D<T>>) -> Point2D<T> {
        let theta = T::from(angle.to_radians()).unwrap();
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        let center = center.unwrap_or( Point2D::origin());
        
        let new_x = center.x + (self.x - center.x) * cos_theta - (self.y - center.y) * sin_theta;
        let new_y = center.y + (self.x - center.x) * sin_theta + (self.y - center.y) * cos_theta;

        Point2D { x: new_x, y: new_y }
    }
    /// rotates the point around origin
    pub fn rotate_origin(&self, angle: f64) -> Point2D<T> {
        self.rotate(angle, None)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use approx::assert_relative_eq;
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

    #[test]
    fn test_point2d_rotation_around_origin() {
        let p = Point2D { x: 1.0, y: 0.0 };

        //90° rotation should rotate (1,0) to (0,1)
        let rotated = p.rotate(90.0, None);
        assert_relative_eq!(rotated.x, 0.0, epsilon = 1e-6);
        assert_relative_eq!(rotated.y, 1.0, epsilon = 1e-6);

        //180° rotation
        let rotated = p.rotate(180.0, None);
        assert_relative_eq!(rotated.x, -1.0, epsilon=1e-6);
        assert_relative_eq!(rotated.y, 0.0, epsilon=1e-6);
        
        //270° rotation
        let rotated = p.rotate(270.0, None);
        assert_relative_eq!(rotated.x, 0.0, epsilon=1e-6);
        assert_relative_eq!(rotated.y, -1.0, epsilon=1e-6);

        //360° rotation
        let rotated = p.rotate(360.0, None);
        assert_relative_eq!(rotated.x, 1.0, epsilon=1e-6);
        assert_relative_eq!(rotated.y, 0.0, epsilon=1e-6);
    }

    #[test]
    fn test_point2d_rotation_negative_coordinates() {
        let p = Point2D {x: -1.0, y: -1.0};

        let rotated = p.rotate(180.0, None);

        assert_relative_eq!(rotated.x, 1.0, epsilon=1e-6);
        assert_relative_eq!(rotated.y, 1.0, epsilon=1e-6);
    }
    #[test]
    fn test_point2d_rotation_around_custom_center() {
        let p = Point2D { x: 2.0, y: 2.0 };
        let center = Point2D {x: 1.0, y: 1.0 };
        

        let rotated = p.rotate(90.0, Some(center));
        assert_relative_eq!(rotated.x, 0.0, epsilon=1e-6);
        assert_relative_eq!(rotated.y, 2.0, epsilon=1e-6);

        let rotated = p.rotate(180.0, Some(center));
        assert_relative_eq!(rotated.x, 0.0, epsilon=1e-6);
        assert_relative_eq!(rotated.y, 0.0, epsilon=1e-6);
    }
    #[test]
    fn test_normalization_nonzero_vector() {
        let p = Point2D {x: 3.0, y: 4.0};
        let normalized = p.normalize().unwrap();
        
        //expected unit vector (0.6, 0.8)
        assert_relative_eq!(normalized.x, 0.6, epsilon=1e-6);
        assert_relative_eq!(normalized.y, 0.8, epsilon=1e-6);
    }

    #[test]
    fn test_point2d_normalize_unit_vector() {
        let p = Point2D {x: 1.0, y: 0.0};
        let normalized = p.normalize().unwrap();
        
        assert_relative_eq!(normalized.x, 1.0, epsilon=1e-6);
        assert_relative_eq!(normalized.y, 0.0, epsilon=1e-6);
    }

    #[test]
    fn test_point2d_normalize_zero_vector() {
        let p =  Point2D {x: 0.0, y: 0.0};
        assert!(p.normalize().is_none(), "Expected None: Cannot normalize zero vector");
    }
}
