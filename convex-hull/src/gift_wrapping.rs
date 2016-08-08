//! This module implements gift-wrapping convex hull algorithm.
//! The module implements the algorithm using two different approach. One adopts
//! classical looping and the other makes use of idiomatic rust through iterator
//! adaptors.
//!
//! # Examples
//!
//! Classic implementation is called as follows.
//!
//! ```
//! use convex_hull::gift_wrapping::*;
//! let v  = vec![(0.0, 0.0), (0.0, 3.0), (4.0, 4.0), (1.0, 3.0), (0.0, 1.0),
//!           (3.0, 6.0), (-3.0, 6.0), (-4.0, 4.0), (1.0, 5.0), (-1.0, 5.0)];
//! let convex_hull = do_gift_wrapping_classic(&v);
//! assert_eq!(convex_hull, Some(vec![&(0.0, 0.0), &(4.0, 4.0),
//! &(3.0, 6.0), &(-3.0, 6.0), &(-4.0, 4.0)]));
//! ```
//!
//! Idiomatic implementation does the same job, but its implementation
//! is based on rust iterator adaptors.
//!
//! ```
//! use convex_hull::gift_wrapping::*;
//! let v  = vec![(0.0, 0.0), (0.0, 3.0), (4.0, 4.0), (1.0, 3.0), (0.0, 1.0),
//!           (3.0, 6.0), (-3.0, 6.0), (-4.0, 4.0), (1.0, 5.0), (-1.0, 5.0)];
//! let convex_hull = do_gift_wrapping_idiomatic(&v);
//! assert_eq!(convex_hull, Some(vec![&(0.0, 0.0), &(4.0, 4.0),
//! &(3.0, 6.0), &(-3.0, 6.0), &(-4.0, 4.0)]));
//! ```

/// Classical implementation of gift-wrapping algorithm.
/// Finds convex hull for given points.
///
/// # Examples
///
/// Classic implementation is called as follows.
///
/// ```
/// use convex_hull::gift_wrapping::*;
/// let v  = vec![(0.0, 0.0), (0.0, 4.0), (4.0, 4.0), (1.0, 4.0), (0.0, 2.0),
///           (3.0, 6.0), (-3.0, 6.0), (-4.0, 4.0), (1.0, 5.4), (-1.0, 3.0)];
/// let convex_hull = do_gift_wrapping_classic(&v);
/// assert_eq!(convex_hull, Some(vec![&(0.0, 0.0), &(4.0, 4.0),
/// &(3.0, 6.0), &(-3.0, 6.0), &(-4.0, 4.0)]));
/// ```
pub fn do_gift_wrapping_classic(points: &[(f32, f32)])
                    -> Option<Vec<&(f32, f32)>>  {
    /*
     * Return `None` if there is no point.
     */
    if points.len() < 1 {
        return None;
    }

    /*
     * Handle the case that there is only one point.
     */
    let mut convex = vec![];
    if points.len() < 2 {
        for p in points {
            convex.push(p);
        }
        return Some(convex);
    }

    /*
     * Start from the bottom point to find out points that form the convex.
     */

    // Cross product between OA and OB vectors, O being
    // the anchor point, A is the best guess so far and b is the
    // current guess.
    let cross = |o: &(f32, f32), a: &(f32, f32), b: &(f32, f32)|
    {(a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)};

    let bottom = match find_bottommost_point(&points) {
        Some(bottommost) => bottommost,
        _                => panic!("Bottommost found None!"),
    };
    let mut o = bottom;
    convex.push(o);
    loop {
        /*
         * This is the classic approach.
         */

        let mut a = &points[0];
        for b in &points[1..] {
            if o == b {
                continue;
            }
            if o == a {
                a = b;
                continue;
            }
            let c = cross(o, a, b);
            if c < 0.0f32 {
                a = b;
            }
        }
        o = a;
        if o == bottom {
            // We are all the way back to the bottom start point.
            break;
        }
        // Found a new point on convex hull.
        convex.push(o);
    }
    Some(convex)
}

/// Idiomatic rust implementation of gift-wrapping algorithm.
/// Finds convex hull for given points.
///
/// # Examples
///
/// Idiomatic implementation is called as follows.
///
/// ```
/// use convex_hull::gift_wrapping::*;
/// let v  = vec![(0.0, 0.0), (0.0, 4.0), (4.0, 4.0), (1.0, 4.0), (0.0, 2.0),
///           (3.0, 6.0), (-3.0, 6.0), (-4.0, 4.0), (1.0, 5.4), (-1.0, 3.0)];
/// let convex_hull = do_gift_wrapping_idiomatic(&v);
/// assert_eq!(convex_hull, Some(vec![&(0.0, 0.0), &(4.0, 4.0),
/// &(3.0, 6.0), &(-3.0, 6.0), &(-4.0, 4.0)]));
/// ```
pub fn do_gift_wrapping_idiomatic(points: &[(f32, f32)])
                            -> Option<Vec<&(f32, f32)>>  {
    /*
     * Return `None` if there is no point.
     */
    if points.len() < 1 {
        return None;
    }

    /*
     * Handle the case that there is only one point.
     */
    let mut convex = vec![];
    if points.len() < 2 {
        for p in points {
            convex.push(p);
        }
        return Some(convex);
    }

    /*
     * Start from the bottom point to find out points that form the convex.
     */

    // Cross product between OA and OB vectors, O being
    // the anchor point, A is the best guess so far and b is the
    // current guess.
    let cross = |o: &(f32, f32), a: &(f32, f32), b: &(f32, f32)|
    {(a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)};

    let bottom = match find_bottommost_point(&points) {
        Some(bottommost) => bottommost,
        _                => panic!("Bottommost found None!"),
    };
    let mut o = bottom;
    convex.push(o);
    loop {
        /*
         * This sounds more rust style.
         */
        o = points.iter()
            .skip(1)
            .filter(|&b| o != b)
            .fold(&points[0], |a, b| {
                if o == a {return b;}
                let c = cross(o, a, b);
                if c < 0.0f32 {b} else {a}
            });

        if o == bottom {
            // We are all the way back to the bottom start point.
            break;
        }
        // Found a new point on convex hull.
        convex.push(o);
    }
    Some(convex)
}

fn find_bottommost_point(points: &[(f32, f32)]) -> Option<&(f32, f32)> {
    if points.len() < 1 {
        None
    } else {
        let mut b = &points[0];
        for p in &points[1..] {
            if b.1 > p.1 {
                b = p;
            }
        }
        Some(b)
    }
}

// Unit tests for the module.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_gift_wrapping_classic() {
        let v  = vec![(0.0, 0.0), (0.0, 4.0), (4.0, 4.0), (1.0, 4.0), (0.0, 2.0),
                (3.0, 6.0), (-3.0, 6.0), (-4.0, 4.0), (1.0, 5.4), (-1.0, 3.0)];
        let convex_hull = do_gift_wrapping_classic(&v);
        assert_eq!(convex_hull, Some(vec![&(0.0, 0.0), &(4.0, 4.0),
        &(3.0, 6.0), &(-3.0, 6.0), &(-4.0, 4.0)]));
    }

    #[test]
    fn test_do_gift_wrapping_idiomatic() {
        let v  = vec![(0.0, 0.0), (0.0, 4.0), (4.0, 4.0), (1.0, 4.0), (0.0, 2.0),
                (3.0, 6.0), (-3.0, 6.0), (-4.0, 4.0), (1.0, 5.4), (-1.0, 3.0)];
        let convex_hull = do_gift_wrapping_idiomatic(&v);
        assert_eq!(convex_hull, Some(vec![&(0.0, 0.0), &(4.0, 4.0),
        &(3.0, 6.0), &(-3.0, 6.0), &(-4.0, 4.0)]));
    }
}
