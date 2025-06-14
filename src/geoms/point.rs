use regex::Regex;


#[derive(Debug, Copy, Clone)]
/// Struct for a 2D Point
pub struct Point {
    /// x coordinate of the point
    x: f64,
    /// y coordinate of the point
    y: f64,
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
    let x: f64 = caps.get(1).ok_or("Missing X coordinate")?.as_str().parse().map_err(|_| "Invalid X value")?;
    let y: f64 = caps.get(2).ok_or("Missing Y coordinate")?.as_str().parse().map_err(|_| "Invalid Y value")?;
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
    /// rotate the point around another point by an angle 
    pub fn rotate(&mut self, rotation_center: Point, angle: f64) {
        todo!()
    }
    /// calculate the 2D distance to another point
    pub fn distance2D(&self, another: &Point) -> f64 {
        ((self.x - another.x).powi(2) + (self.y - another.y).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_origin() {
        let result = Point::origin();
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
    }
    #[test]
    fn test_get_coords_origin() {
        let result = Point::origin();
        assert_eq!(result.get_x(),0.0);
        assert_eq!(result.get_y(),0.0);

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
        assert_eq!(result.get_x(),-5.8);
        assert_eq!(result.get_y(),0.657);

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
        assert_eq!(result.get_x(),-3.14);
        assert_eq!(result.get_y(),9.1548595);

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
        assert!((dist - 5.0).abs() < 1e-10); // Expected distance: 5.0
    }

    #[test]
    fn test_distance_negative_coords() {
        let p1 = Point { x: -1.0, y: -2.0 };
        let p2 = Point { x: -4.0, y: -6.0 };
        let dist = p1.distance2D(&p2);
        assert!((dist - 5.0).abs() < 1e-10); // Expected distance: 5.0
    }

    #[test]
    fn test_distance_same_point() {
        let p = Point { x: 3.14, y: 2.71 };
        let dist = p.distance2D(&p);
        assert!((dist - 0.0).abs() < 1e-10); // Expected distance: 0.0
    }

    #[test]
    fn test_distance_x_axis_aligned() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 0.0, y: 5.0 };
        let dist = p1.distance2D(&p2);
        assert!((dist - 5.0).abs() < 1e-10); // Expected distance: 5.0
    }

    #[test]
    fn test_distance_y_axis_aligned() {
        let p1 = Point { x: 76.54, y: 850.0 };
        let p2 = Point { x: 98.213213, y: 850.0 };
        let dist = p1.distance2D(&p2);
        assert!((dist - 21.673213).abs() < 1e-10); // Expected distance: 5.0
    }

    // tests with distance and rotate
}