//! This module parses points. The module provides convex hull algorithms with a
//! vector of two-element f32 tuples, each tuple representing a point.

use std::io;
use std::io::prelude::*;
// This is how we do macros in Rust.
// This provides similar functionality as `sscanf` in C.
macro_rules! sscan {
    ($string:expr, $trim:expr, $sep:expr, $( $x:ty ),+) => {{
        let mut iter = $string.trim_matches($trim).split($sep);
        ($(iter.next().and_then(|word| word.trim().parse::<$x>().ok())
           .unwrap(),)*)
    }}
}

/// Reads and parse `f32` points from standard input.
pub fn get_points_from_stdin() -> Result<(Vec<(f32, f32)>), String> {
   /*
    * Parse input to extract points.
    */
    let mut points: Vec<(f32, f32)> = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let trim: &[_] = &[' ', '(', ')'];
        let string = line.unwrap();
        let point = sscan!(string, trim, ',', f32, f32);
        points.push(point);
    }
    if points.len() > 0 {
        Ok(points)
    } else {
        Err("Can not parse points from stdin".to_owned())
    }
}

