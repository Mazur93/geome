mod geoms;
//use geoms::Point;
use geoms::line::Line;
use geoms::point::Point;
use std::time::Instant;

fn main() {
    println!("Hello!");
    let a = Point::origin();
    println!("a: {a:?}");
    println!("{0}", a.get_x());
    let b = Point::from_wkt("POINT (-65 7.5875)");
    let b_point = b.unwrap();
    let x = b_point.get_x();
    let y = b_point.get_y();
    println!("{}", b_point.to_wkt());
    //let gh = Li

    let mut p1 = Point::new(1.0, 0.0);
    let origin = Point::origin();
    println!("{:?}", p1);
    p1.rotate(&origin, 278.4857, false);
    println!("{:?}", p1);
    println!("{}", p1 != b_point);

    let start = Instant::now();

    let random_points = Point::generate_random_points(2500, -4.0, -40.87, 12.9, 7043.4, 2);
    let duration = start.elapsed();
    println!("Execution time point generation: {:?}", duration);
    for point in &random_points[0..5] {
        println!("{:?}", point);
    }
    let start = Instant::now();
    let results = Point::pairwise_distances(&random_points);
    let duration = start.elapsed();
    println!("Execution time distance matrix: {:?}", duration);
    //println!("{:?}", results);
}
