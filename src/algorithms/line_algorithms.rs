use num::{Num, Float};

pub fn are_parallel_lines<T: Num + Copy + PartialOrd>(l1: &Line2D<T>, l2: &Line2D<T>) -> bool {
    let (Point2D { x: x1, y: y1 }, Point2D { x: x2, y: y2 }) = l1;
    let (Point2D { x: x3, y: y3 }, Point2D { x: x4, y: y4 }) = l2;

    let lhs = (y2 - y1) * (x4 - x3);
    let rhs = (x2 - x1) * (y4 - y3);

    if let Some(epsilon) = T::from(1e-6) {
        (lhs - rhs).abs() < epsilon
    } else {
        lhs == rhs
    }
}

pub fn are_perpendicular_lines<T: Num + Copy + PartialOrd>(l1: &Line2D<T>, l2: &Line2D<T>) -> bool {
    let (Point2D {x: x1, y: y1}, Point2D {x: x2, y: y2}) = l1;
    let (Point2D {x: x3, y: y3}, Point2D {x: x4, y: y4}) = l2;

    let dot_product = (x2 - x1) * (x4 - x3) + (y2 - y1) * (y4 - y3);

    if let Some(epsilon) = T::from(1e-6) {
        dot_product.abs() < epsilon
    } else {
        dot_product == T::zero()
    }
}

pub fn intersection_point<T: Num + Copy + PartialOrd>(l1: &Line2D<T>, l2: &Line2D<T>) -> Option<Point2D<T>> {
    let (Point2D{x: x1, y: y1}, Point2D{x: x2, y: y2}) = l1;
    let (Point2d{x: x3, y: y3}, Point2D{x: x4, y: y4}) = l2;

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    
    if let Some(epsilon) = T::from(1e-6) {
        dot_product.abs() < epsilon
    } else {
        dot_product == T::zero()
    }

    let det1 = (x1 * y2 - y1 * x2);
    let det2 = (x3 * y4 - y3 * x4);

    let intersect_x = (det1 * (x3 - x4) - det2 * (x1 - x2)) / denom;
    let intersect_y = (det1 * (y3 - y4) - det2 * (y1 - y2)) / denom;

    Some(Point2D::new(intersect_x, intersect_y))
}

/// Calculates the angle (in radians) between two `Line2D` objects using the cosine formula.
///
/// The angle θ between two vectors A and B is given by:
///
/// ```text
///           A • B
/// cosθ = -------------
///         |A| * |B|
/// ```
///
/// where:
/// - `A • B` is the dot product of the direction vectors of `line1` and `line2`.
/// - `|A|` and `|B|` are the magnitudes (lengths) of the direction vectors.
///
/// # Parameters
/// - `line1`: A reference to the first `Line2D<T>`.
/// - `line2`: A reference to the second `Line2D<T>`.
///
/// # Returns
/// - `Some(T)`: The angle in radians if the computation is valid.
/// - `None`: If either line has a zero-length direction vector, making the computation undefined.
///
/// # Type Parameters
/// - `T`: A floating-point type that implements `Float` and `Copy`.
pub fn angle_between<T: Float + Copy>(line1: &Line2D<T>, line2: &Line2D<T>) -> Option<T> {
    let (Point2D{x: x1, y: y1}, Point2D{x: x2, y: y2}) = line1;
    let (Point2D{x: x3, y: y3}, Point2D{x: x4, y: y4}) = line2;

    let dot_product = (x2 - x1) * (x4 - x3) + (y2 - y1) * (y4 - y3);

    let mag1 = ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt();
    let mag2 = ((x4 - x3) * (x4 - x3) + (y4 - y3) * (y4 - y3)).sqrt();

    if mag1.is_zero() || mag2.is_zero() {
        return None;
    }

    Some((dot_product / (mag1 * mag2)).acos())
}