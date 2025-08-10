use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use regex::Regex;

#[derive(Debug, Copy, Clone)]
/// Struct for a 2D Point
pub struct Point {
    /// x coordinate of the point
    x: f64,
    /// y coordinate of the point
    y: f64,
}

// Implement trait PartialEq
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    /// create a point with given coordinates
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    /// create a default point with coordinates x: 0.0 and y: 0.0
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    /// create Point from WKT
    pub fn from_wkt(wkt_string: &str) -> Result<Point, &'static str> {
        let re = Regex::new(r"(?i)point\s*\(\s*([+-]?\d+\.?\d*)\s+([+-]?\d+\.?\d*)\s*\)").unwrap(); // (?i)point\s*\(\s*([+-]?\d*\.?\d+)\s+([+-]?\d*\.?\d+)\s*\)
        let caps = re.captures(wkt_string.trim()).ok_or("Invalid WKT format")?;
        let x: f64 = caps
            .get(1)
            .ok_or("Missing X coordinate")?
            .as_str()
            .parse()
            .map_err(|_| "Invalid X value")?;
        let y: f64 = caps
            .get(2)
            .ok_or("Missing Y coordinate")?
            .as_str()
            .parse()
            .map_err(|_| "Invalid Y value")?;
        Ok(Point { x, y })
    }

    /// get WKT (well-known text) representation of a 2D point
    pub fn to_wkt(&self) -> String {
        format!("POINT ({} {})", self.x, self.y)
    }

    /// get the x coordinate of the point
    pub fn get_x(&self) -> f64 {
        self.x
    }
    /// get the y coordinate of the point
    pub fn get_y(&self) -> f64 {
        self.y
    }

    /// set the x coordinate of the point
    pub fn set_x(&mut self, new_x: f64) {
        self.x = new_x;
    }
    /// set the y coordinate of the point
    pub fn set_y(&mut self, new_y: f64) {
        self.y = new_y;
    }

    /// rotate the point around another point by an angle. Positive angles are counter-clockwise and negative angles are clockwise.
    /// The angle can be given in degrees or radians, depending on the `use_radians` parameter.
    pub fn rotate(&mut self, rotation_center: &Point, angle: f64, use_radians: bool) {
        // Check if the angle is in radians or degrees, set cosine and sine
        let theta = if use_radians {
            angle
        } else {
            angle.to_radians()
        };
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        // Translate point to origin
        let dx = self.x - rotation_center.x;
        let dy = self.y - rotation_center.y;

        // Apply rotation
        let x_new = dx * cos_theta - dy * sin_theta;
        let y_new = dx * sin_theta + dy * cos_theta;

        // Translate back
        self.x = rotation_center.x + x_new;
        self.y = rotation_center.y + y_new;
    }
    /// calculate the 2D distance to another point
    pub fn distance2D(&self, another: &Point) -> f64 {
        ((self.x - another.x).powi(2) + (self.y - another.y).powi(2)).sqrt()
    }

    // something like a distance matrix
    pub fn pairwise_distances(points_vector: &[Point]) -> Vec<Vec<f64>> {
        let n = points_vector.len();
        let mut results = vec![vec![0.0; n]; n];
            
        for i in 0..(n-1) {
            let pt_i = &points_vector[i];
            for j in (i+1)..n{
                let dist_temp = pt_i.distance2D(&points_vector[j]);
                    results[i][j] = dist_temp;
                    results[j][i] = dist_temp;
            }
        }
         results
    }

    /// Generate a vector of n (pseudo) random points given min/max values for x and y. Upper bounds are inclusive (..=).
    pub fn generate_random_points(
        n: i32,
        min_x: f64,
        min_y: f64,
        max_x: f64,
        max_y: f64,
        seed: u64,
    ) -> Vec<Point> {
        // check range validity
        if min_x > max_x || min_y > max_y {
            panic!("Range is not valid, min values cannot be greater than max values!")
        }

        // use a pseudo random generator for reproducible results
        let mut rng = StdRng::seed_from_u64(seed);
        let mut vec_pts: Vec<Point> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let x = rng.random_range(min_x..=max_x);
            let y = rng.random_range(min_y..=max_y);
            vec_pts.push(Point::new(x, y));
        }
        vec_pts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    #[test]
    fn test_creation_origin() {
        let result = Point::origin();
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
    }

    #[test]
    fn test_set_x_y() {
        let mut result = Point::origin();
        result.set_x(9.0);
        result.set_y(-3.14);
        assert_eq!(result.x, 9.0);
        assert_eq!(result.y, -3.14);
    }

    #[test]
    fn test_get_coords_origin() {
        let result = Point::origin();
        assert_eq!(result.get_x(), 0.0);
        assert_eq!(result.get_y(), 0.0);
    }
    #[test]
    fn test_creation_new() {
        let result = Point::new(14.45, 4.312);
        assert_eq!(result.x, 14.45);
        assert_eq!(result.y, 4.312);
    }
    #[test]
    fn test_get_coords_new() {
        let result = Point::new(-5.8, 0.657);
        assert_eq!(result.get_x(), -5.8);
        assert_eq!(result.get_y(), 0.657);
    }
    #[test]
    fn test_creation_wkt_int() {
        let result = Point::from_wkt("POINT (34 2)").unwrap();
        assert_eq!(result.x, 34.0);
        assert_eq!(result.y, 2.0);
    }

    #[test]
    fn test_creation_wkt_small_int() {
        let result = Point::from_wkt("point (34 2)").unwrap();
        assert_eq!(result.x, 34.0);
        assert_eq!(result.y, 2.0);
    }

    #[test]
    fn test_creation_wkt_dbl() {
        let result = Point::from_wkt("POINT (34.98 -8.543)").unwrap();
        assert_eq!(result.x, 34.98);
        assert_eq!(result.y, -8.543);
    }

    #[test]
    fn test_get_coords_wkt() {
        let result = Point::from_wkt("POINT (-3.14 9.1548595)").unwrap();
        assert_eq!(result.get_x(), -3.14);
        assert_eq!(result.get_y(), 9.1548595);
    }
    #[test]
    fn test_creation_wkt_fail_comma() {
        let result = Point::from_wkt("POINT (34, 2)");
        assert!(result.is_err());
    }
    #[test]
    fn test_creation_wkt_fail_invalid_wkt() {
        let wkt = "POINTS ( 5 4)";
        let result = Point::from_wkt(wkt);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid WKT format");
    }

    #[test]
    fn test_creation_wkt_fail_invalid_x() {
        let wkt = "POINT (test 2)";
        let result = Point::from_wkt(wkt);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid WKT format");
    }
    #[test]
    fn test_creation_wkt_fail_invalid_y() {
        let wkt = "POINT (98 test)";
        let result = Point::from_wkt(wkt);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid WKT format");
    }
    #[test]
    fn test_to_wkt_origin() {
        let orig = Point::origin();
        assert_eq!("POINT (0 0)", orig.to_wkt());
    }

    #[test]
    fn test_to_wkt_other_point() {
        let orig = Point::new(-9.4874, 67598.58548);
        assert_eq!("POINT (-9.4874 67598.58548)", orig.to_wkt());
    }

    #[test]
    fn test_to_wkt_other_point2() {
        let orig = Point::new(2.0, 67598.58548);
        assert_eq!("POINT (2 67598.58548)", orig.to_wkt());
    }

    // tests for distance calculations
    #[test]
    fn test_distance_positive_coords() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = Point { x: 4.0, y: 6.0 };
        let dist = p1.distance2D(&p2);
        assert!((dist - 5.0).abs() < EPS); // Expected distance: 5.0
    }

    #[test]
    fn test_distance_negative_coords() {
        let p1 = Point { x: -1.0, y: -2.0 };
        let p2 = Point { x: -4.0, y: -6.0 };
        let dist = p1.distance2D(&p2);
        assert!((dist - 5.0).abs() < EPS); // Expected distance: 5.0
    }

    #[test]
    fn test_distance_same_point() {
        let p = Point { x: 3.14, y: 2.71 };
        let dist = p.distance2D(&p);
        assert!((dist - 0.0).abs() < EPS); // Expected distance: 0.0
    }

    #[test]
    fn test_distance_x_axis_aligned() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 0.0, y: 5.0 };
        let dist = p1.distance2D(&p2);
        assert!((dist - 5.0).abs() < EPS); // Expected distance: 5.0
    }

    #[test]
    fn test_distance_y_axis_aligned() {
        let p1 = Point { x: 76.54, y: 850.0 };
        let p2 = Point {
            x: 98.213213,
            y: 850.0,
        };
        let dist = p1.distance2D(&p2);
        assert!((dist - 21.673213).abs() < EPS); // Expected distance: 5.0
    }

    // tests rotate
    #[test]
    fn test_rotate_zero_angle_degrees() {
        let mut p1 = Point { x: 1.0, y: 2.0 };
        let center = Point { x: 0.0, y: 0.0 };
        p1.rotate(&center, 0.0, false);
        assert_eq!(p1.x, 1.0);
        assert_eq!(p1.y, 2.0);
    }

    #[test]
    fn test_rotate_zero_angle_radians() {
        let mut p1 = Point { x: 1.0, y: 2.0 };
        let center = Point { x: 0.0, y: 0.0 };
        p1.rotate(&center, -0.0, true);
        assert_eq!(p1.x, 1.0);
        assert_eq!(p1.y, 2.0);
    }

    #[test]
    fn test_rotate_full_circle_degrees() {
        let mut p1 = Point { x: 1.0, y: 2.0 };
        let center = Point { x: 0.0, y: 0.0 };
        p1.rotate(&center, 360.0, false);
        assert!((p1.x - 1.0).abs() < EPS);
        assert!((p1.y - 2.0).abs() < EPS);
    }

    #[test]
    fn test_rotate_negative_angle() {
        let mut p1 = Point { x: 1.0, y: 0.0 };
        let center = Point { x: 0.0, y: 0.0 };
        p1.rotate(&center, -90.0, false);
        assert!((p1.x - 0.0).abs() < EPS);
        assert!((p1.y + 1.0).abs() < EPS);
    }

    #[test]
    fn test_rotate_around_self() {
        let mut p1 = Point { x: 3.0, y: 4.0 };
        let center = Point { x: 3.0, y: 4.0 };
        p1.rotate(&center, 45.0, false);
        assert_eq!(p1.x, 3.0);
        assert_eq!(p1.y, 4.0);
    }

    #[test]
    fn test_rotate_non_45_multiple_angle() {
        let mut p1 = Point { x: 2.0, y: 0.0 };
        let center = Point { x: 0.0, y: 0.0 };
        p1.rotate(&center, 30.0, false);
        let expected_x = 2.0 * (30.0f64.to_radians().cos());
        let expected_y = 2.0 * (30.0f64.to_radians().sin());
        assert!((p1.x - expected_x).abs() < EPS);
        assert!((p1.y - expected_y).abs() < EPS);
    }

    #[test]
    fn test_arbitrary_angle_123_degrees() {
        let mut p1 = Point { x: 1.0, y: 0.0 };
        let center = Point { x: 0.0, y: 0.0 };
        p1.rotate(&center, 123.0, false);
        let rad = 123.0f64.to_radians();
        let expected_x = rad.cos();
        let expected_y = rad.sin();
        assert!((p1.x - expected_x).abs() < EPS);
        assert!((p1.y - expected_y).abs() < EPS);
    }

    #[test]
    fn test_arbitrary_center() {
        let mut p1 = Point { x: 2.0, y: 3.0 };
        let center = Point { x: 1.0, y: 1.0 };
        p1.rotate(&center, 90.0, false);
        // (2,3) around (1,1) by 90°: (x',y') = (1-(y-1), 1+(x-1)) => (-1,2)
        assert!((p1.x + 1.0).abs() < EPS);
        assert!((p1.y - 2.0).abs() < EPS);
    }
    // test for == operator
    #[test]
    fn test_points_are_equal() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = Point { x: 1.0, y: 2.0 };
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_points_are_not_equal() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = Point { x: 2.0, y: 3.0 };
        assert_ne!(p1, p2);
    }

    // test random points generation
    #[test]
    fn test_generate_random_points_count_and_bounds() {
        let n = 100;
        let min_x = -1.0;
        let max_x = 5.0;
        let min_y = -2.0;
        let max_y = 2.0;
        let seed = 42;

        let points = Point::generate_random_points(n, min_x, min_y, max_x, max_y, seed);

        assert_eq!(points.len(), n as usize);

        for p in &points {
            assert!(p.x >= min_x && p.x <= max_x, "x out of bounds: {}", p.x);
            assert!(p.y >= min_y && p.y <= max_y, "y out of bounds: {}", p.y);
        }
    }

    #[test]
    fn test_generate_random_points_deterministic() {
        let n = 10;
        let min_x = -985.123;
        let max_x = 10.0;
        let min_y = 0.0;
        let max_y = 10.0;
        let seed = 1954;

        let points1 = Point::generate_random_points(n, min_x, min_y, max_x, max_y, seed);
        let points2 = Point::generate_random_points(n, min_x, min_y, max_x, max_y, seed);

        assert_eq!(
            points1, points2,
            "Points generated with the same seed should be equal"
        );
    }

    #[test]
    #[should_panic(expected = "Range is not valid, min values cannot be greater than max values!")]
    fn test_generate_random_points_panics_on_invalid_range() {
        // min_x > max_x triggers the panic
        Point::generate_random_points(120, 5.0, 0.0, 1.0, 10.0, 154);
    }
}
