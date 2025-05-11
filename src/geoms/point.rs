/// Struct for a 2D Point
#[derive(Debug)]
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
    pub fn from_WKT(wkt_string: &str) -> Result<Point, &'static str> {
        todo!()
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
    pub fn distance2D(&self, another: Point) -> f64 {
        todo!()
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
        let result = Point::from_WKT("POINT (34 2)").unwrap();
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
    }

    #[test]
    fn test_creation_wkt_small_int() {
        let result = Point::from_WKT("point (34 2)").unwrap();
        assert_eq!(result.x, 34.0);
        assert_eq!(result.y, 2.0);
    }

    #[test]
    fn test_creation_wkt_dbl() {
        let result = Point::from_WKT("POINT (34.98 -8.543)").unwrap();
        assert_eq!(result.x, 34.98);
        assert_eq!(result.y, -8.543);
    }


    #[test]
    fn test_get_coords_wkt() {
        let result = Point::from_WKT("POINT (-3.14 9.15)").unwrap();
        assert_eq!(result.get_x(),0.0);
        assert_eq!(result.get_y(),0.0);

    }
    #[test]
    fn test_creation_wkt_fail() {
        let result = Point::from_WKT("POINT (34, 2)");
        assert!(result.is_err());
    }
}