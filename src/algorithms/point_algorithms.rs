use std::cmp::Ordering;
use num::{Float};
use std::fmt::Debug;
use crate::point::Point2D;
///Computes the centorid of a set of points
pub fn centroid_2d<T: Float + Debug>(points: &[Point2D<T>]) -> Point2D<T> {
    let n = points.len();
    let sum = points.iter().cloned().fold(Point2D::origin(), |acc, p| acc + p);
    sum / T::from(n).unwrap()
}

///Computes the axis aligned bounding box for a set of points
pub fn bounding_box_2d<T: Float + Debug>(points: &[Point2D<T>]) -> Option<(Point2D<T>, Point2D<T>)> {
    if points.is_empty() {
        return None;
    }

    let mut max_x = T::min_value();
    let mut max_y = T::min_value();
    let mut min_x = T::max_value();
    let mut min_y = T::max_value();

    for &point in points {
        if point.x < min_x { min_x = point.x; }
        if point.y < min_y { min_y = point.y; }
        if point.x > max_x { max_x = point.x; }
        if point.y > max_y { max_y = point.y; }
    }
    Some((Point2D::new(min_x, min_y), Point2D::new(max_x, max_y)))
}

/// Computes the convex hull of a set of 2D points using Andrew's Monotone Chain Algorithm.
///
/// # Arguments
/// * `points` - A slice of `Point2D<T>` representing the set of points.
///
/// # Returns
/// * A `Vec<Point2D<T>>` representing the convex hull in counterclockwise order.
///
/// # Algorithm Explanation
/// 1. Sorts points lexicographically (by x, then y).
/// 2. Constructs the **lower hull**:
///     - Iterates through points and removes concave points using the cross-product test.
/// 3. Constructs the **upper hull** (same process but in reverse order).
/// 4. Combines both hulls and removes duplicate points.
///
/// # Time Complexity
/// **O(n log n)** due to sorting, but hull construction runs in **O(n)**, making this optimal.
pub fn convex_hull_2d<T: Float + Debug>(points: &[Point2D<T>]) -> Vec<Point2D<T>> {
    let mut points = points.to_vec();

    if points.len() < 3 {
        return points;
    }
    
    points.sort_by(|a, b| {
                        a.x
                        .partial_cmp(&b.x)
                        .unwrap_or(Ordering::Equal)
                        .then(
                            a.y
                            .partial_cmp(&b.y)
                            .unwrap_or(Ordering::Equal)
                        )
    });

    let cross_product = |o: Point2D<T>, a: Point2D<T>, b: Point2D<T>| -> T {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    };

    let mut lower = Vec::new();
    for &p in points.iter() {
        while lower.len() >= 2 && cross_product(lower[lower.len()-2], lower[lower.len()-1], p) <= T::zero() {
            lower.pop();
        }
        lower.push(p);
    }

    let mut upper = Vec::new();
    for &p in points.iter().rev() {
        while upper.len() >= 2 && cross_product(upper[upper.len() - 2], upper[upper.len()-1], p) <= T::zero() {
            upper.pop();
        }
        upper.push(p);
    }

    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point2D;
    #[test]
    fn test_convex_hull_2d_single_point() {
        let points = vec![Point2D::new(1.0, 1.0)];
        let hull = convex_hull_2d(&points);
        assert_eq!(hull, points);
    }

    #[test]
    fn test_convex_hull_2d_two_points() {
        let points = vec![Point2D::new(1.0, 1.0), Point2D::new(1.0, 2.0)];
        let hull = convex_hull_2d(&points);
        assert_eq!(hull, points);
    }

    #[test]
    fn test_convex_hull_2d_five_points() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, -4.0),
            Point2D::new(-1.0, -5.0),
            Point2D::new(-5.0, -3.0),
            Point2D::new(-3.0, -1.0),
            Point2D::new(-1.0, -3.0),
            Point2D::new(-2.0, -2.0),
            Point2D::new(-1.0, -1.0),
            Point2D::new(-2.0, -1.0),
            Point2D::new(-1.0, 1.0),
        ];
        let expected_hull = vec![
            Point2D::new(-5.0, -3.0),
            Point2D::new(-1.0, -5.0),
            Point2D::new(1.0, -4.0),
            Point2D::new(0.0, 0.0),
            Point2D::new(-1.0, 1.0),
        ];

        let hull = convex_hull_2d(&points);
        assert_eq!(hull, expected_hull);
    }

    #[test]
    fn test_centroid_2d_single_point() {
        let points = vec![Point2D::new(1.0, 0.0)];
        let centroid = centroid_2d(&points);
        assert_eq!(centroid, points[0]);
    }
    
    #[test]
    fn test_centroid_2d_multiple_points() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(2.0, 3.0),
            Point2D::new(1.0, 4.0)
        ];
        let centroid = centroid_2d(&points);

        assert_eq!(centroid, Point2D::new(1.0, 2.0));
    }

    #[test]
    fn test_bounding_box_2d_empty() {
        let points: Vec<Point2D<f64>> = vec![];
        let bbox = bounding_box_2d(&points);
        assert!(bbox.is_none(), "Expected None: Empty points return None");
    }

    #[test]
    fn test_bounding_box_2d_single_point() {
        let points = vec![Point2D::new(1.0, 2.0)];
        let bbox = bounding_box_2d(&points);
        assert_eq!(bbox, Some((Point2D::new(1.0, 2.0), Point2D::new(1.0, 2.0))));
    }

    #[test]
    fn test_bounding_box_2d_multiple_points() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(2.0, 3.0),
            Point2D::new(1.0, 4.0),
        ];
        let bbox = bounding_box_2d(&points);
        assert_eq!(bbox, Some((Point2D::new(0.0, 0.0), Point2D::new(2.0, 4.0))));
    }
    
    #[test]
    fn test_bounding_box_negative_coordinates() {
        let points = vec![
            Point2D::new(-3.0, -2.0),
            Point2D::new(1.0, -4.0),
            Point2D::new(-1.0, 3.0),
            Point2D::new(2.0, 1.0),
        ];
        let bbox = bounding_box_2d(&points);
        assert_eq!(bbox, Some((Point2D::new(-3.0, -4.0), Point2D::new(2.0, 3.0))));
    }

}