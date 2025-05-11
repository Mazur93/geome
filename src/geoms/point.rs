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
    
    // create a default point with coordinates x: 0.0 and y: 0.0
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // create Point from WKT
    pub fn from_WKT(wkt_string: &str) -> Point {
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
}