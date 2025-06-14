use super::point::Point;

#[derive(Debug)]
/// Struct for a 2D Line
pub struct Line {
    /// points of the line
    points: Vec<Point>
}


impl Line {

    /// create a line with given points
    pub fn new(points: Vec<Point>) -> Self {
        // check if there are at least two points
        if points.len() < 2 {
            panic!("A line must have at least two points.");
        }
        Line {points: points}
    }

    /// get the number of points in the line
    pub fn get_number_of_points(&self) -> usize {
        self.points.len()
    }
    /// get the length of the line
    pub fn get_length(&self) -> f64 {
        let mut length: f64 = 0.0;
        for i in 0..self.points.len() -1 {
            length += self.points[i].distance2D(&self.points[i+1]);
        }
        length
    }
    /// get a copy of the first point of the line
    pub fn get_start(&self) -> Point {
        self.points.first().copied().unwrap()
        //todo!()
    }

    /// get a copy of the last point of the line
    pub fn get_last(&self) -> Point {
        self.points.last().copied().unwrap()
    }

    /// calculate the minimum distance between a point and the line
    pub fn distance_to_point(&self, point: Point) -> f64 {
        todo!();
    }
    
    /// get WKT (well-known text) representation of a 2D point
    pub fn to_wkt(&self) -> String {
        todo!()
    } 

    /// create Line from WKT
    pub fn from_wkt(wkt_string: &str) -> Self {
        todo!()
    }

    /// rotate the line around another point by an angle 
    pub fn rotate(&mut self, rotation_center: Point, angle: f64) {
        todo!()
    }

    /// calculate a point at a given distance from the start of the line
    pub fn interpolate_line_distance(&self, distance:f64) -> Option<Point> {
        todo!()
    }

    /// calculate a point at a given distance (in % between 0 and 1) from the start of the line
    pub fn interpolate_line_percentage(&self, percentange:f64) -> Option<Point> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn line_creation_two_points() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
        ];
        let line = Line::new(points);
        assert_eq!(line.get_number_of_points(), 2);
        assert_eq!(line.get_start().get_x(), 0.0);
        assert_eq!(line.get_start().get_y(), 0.0);
        assert_eq!(line.get_last().get_x(), 1.0);
        assert_eq!(line.get_last().get_y(), 1.0);
    }

    #[test]
    fn line_creation_twenty_points() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 2.0),
            Point::new(3.0, 3.0),
            Point::new(4.0, 4.0),
            Point::new(5.0, 5.0),
            Point::new(6.0, 6.0),
            Point::new(7.0, 7.0),
            Point::new(8.0, 8.0),
            Point::new(9.0, 9.0),
            Point::new(10.0, 10.0),
            Point::new(11.0, 11.0),
            Point::new(12.0, 12.0),
            Point::new(13.0, 13.0),
            Point::new(14.0, 14.0),
            Point::new(15.0, 15.0),
            Point::new(16.0, 16.0),
            Point::new(17.0, 17.0),
            Point::new(18.0, 18.0),
            Point::new(19.0, 19.0),
        ];
        let line = Line::new(points);
        assert_eq!(line.get_number_of_points(), 20);
        assert_eq!(line.get_start().get_x(), 0.0);
        assert_eq!(line.get_start().get_y(), 0.0);
        assert_eq!(line.get_last().get_x(), 19.0);
        assert_eq!(line.get_last().get_y(), 19.0);
    }

    #[test]
    fn test_length_diagonal_and_horizontal() {
        let points = vec![
            Point::new(1.5, 2.5),
            Point::new(4.5, 6.5), // diagonal: sqrt(3^2 + 4^2) = 5
            Point::new(10.5, 6.5), // horizontal: 6.0
        ];
        let line = Line::new(points);
        assert!((line.get_length() - 11.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_length_negative_coordinates() {
        let points = vec![
            Point::new(-2.0, -3.0),
            Point::new(1.0, 1.0),   // diagonal: 5.0
            Point::new(1.0, 4.0),   // vertical: 3.0
            Point::new(-2.0, 4.0),  // horizontal: 3.0
        ];
        let line = Line::new(points);
        assert!((line.get_length() - 11.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_length_floating_points() {
        let points = vec![
            Point::new(0.1, 0.1),
            Point::new(0.4, 0.5), // sqrt(0.3^2 + 0.4^2) = 0.5
            Point::new(1.4, 0.5), // horizontal: 1.0
        ];
        let line = Line::new(points);
        assert!((line.get_length() - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_distance_to_point() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
        ];
        let line = Line::new(points);
        let point = Point::new(2.0, 3.8458);
        let distance = line.distance_to_point(point);
        // Expected distance is 3.0 (perpendicular distance from point to line)
        assert!((distance - 3.8458).abs() < 1e-10);
    }

    #[test]
    fn test_to_wkt() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
        ];
        let line = Line::new(points);
        let wkt = line.to_wkt();
        assert_eq!(wkt, "LINESTRING (0.0 0.0, 1.0 1.0)");
    }

    #[test]
    fn test_from_wkt() {
        let wkt = "LINESTRING (0.0 0.0, 1.0 1.0)";
        let line = Line::from_wkt(wkt);
        assert_eq!(line.get_number_of_points(), 2);
        assert_eq!(line.get_start().get_x(), 0.0);
        assert_eq!(line.get_start().get_y(), 0.0);
        assert_eq!(line.get_last().get_x(), 1.0);
        assert_eq!(line.get_last().get_y(), 1.0);
    }

    #[test]
    fn test_interpolate_line_distance() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
        ];
        let line = Line::new(points);
        let interpolated_point = line.interpolate_line_distance(2.0).unwrap();
        assert_eq!(interpolated_point.get_x(), 2.0);
        assert_eq!(interpolated_point.get_y(), 0.0);
    }

    #[test]
    fn test_interpolate_line_percentage() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
        ];
        let line = Line::new(points);
        let interpolated_point = line.interpolate_line_percentage(0.75).unwrap();
        assert_eq!(interpolated_point.get_x(), 3.0);
        assert_eq!(interpolated_point.get_y(), 0.0);
    }

}