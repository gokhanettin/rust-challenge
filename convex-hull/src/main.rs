extern crate convex_hull;
use convex_hull::gift_wrapping::*;
use convex_hull::point_parser::*;

fn main() {
    let points = match get_points_from_stdin() {
        Ok(points) => points,
        Err(err)   => panic!("Error: {:?}", err),
    };

    let convex = do_gift_wrapping_idiomatic(&points).unwrap();
    for p in convex {
        println!("{:?}", p);
    }
}
