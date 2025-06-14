mod geoms;
//use geoms::Point;
use geoms::point::Point;
use geoms::line::Line;

fn main() {
    println!("Hello!");
    let a = Point::origin();
    println!("a: {a:?}");
    println!("{0}",a.get_x());
    let b = Point::from_wkt("POINT (-65 7.5875)");
    let b_point = b.unwrap();
    let x = b_point.get_x(); 
    let y = b_point.get_y(); 
    println!("{}", b_point.to_wkt());
    //let gh = Li

}