use geomengine::{Point2D};
fn main() {
    let p1 = Point2D::new(3.0, 4.0);
    let p2 = Point2D::origin();
    let dist = p1.distance(&p2);
    println!("Distance between {} and {} is {}", p1, p2, dist);
}