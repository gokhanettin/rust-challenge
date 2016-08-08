// Integration tests go here.
extern crate convex_hull;
use convex_hull::*;

#[test]
fn test_do_gift_wrapping_classic() {
    let v  = vec![(0.0, 0.0), (0.0, 4.0), (4.0, 4.0), (1.0, 4.0), (0.0, 2.0),
            (3.0, 6.0), (-3.0, 6.0), (-4.0, 4.0), (1.0, 5.4), (-1.0, 3.0)];
    let convex_hull = gift_wrapping::do_gift_wrapping_classic(&v);
    assert_eq!(convex_hull, Some(vec![&(0.0, 0.0), &(4.0, 4.0),
    &(3.0, 6.0), &(-3.0, 6.0), &(-4.0, 4.0)]));
}

#[test]
fn test_do_gift_wrapping_idiomatic() {
    let v  = vec![(0.0, 0.0), (0.0, 4.0), (4.0, 4.0), (1.0, 4.0), (0.0, 2.0),
            (3.0, 6.0), (-3.0, 6.0), (-4.0, 4.0), (1.0, 5.4), (-1.0, 3.0)];
    let convex_hull = gift_wrapping::do_gift_wrapping_idiomatic(&v);
    assert_eq!(convex_hull, Some(vec![&(0.0, 0.0), &(4.0, 4.0),
    &(3.0, 6.0), &(-3.0, 6.0), &(-4.0, 4.0)]));
}
