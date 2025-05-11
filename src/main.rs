mod geoms;
//use geoms::Point;
use geoms::point::Point;

fn main() {
    println!("Hello!");
    let a = Point::origin();
    println!("a: {a:?}");
    println!("{0}",a.get_x());
}