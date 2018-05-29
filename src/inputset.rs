//! Collection of functions that operates and
//! modifies the input set to conveniently
//! determine the hull points
use points::*;

///Converts any raw point tuple to Point2D type
/// and pushes it to the inputset
pub fn push(inputset: &mut Vec<Point2D>, point: (f64, f64)) -> &Vec<Point2D> {
    let point = Point2D::new(point.0, point.1);
    inputset.push(point);
    inputset
}

///Finds the point in input set with least y-coordinate or the
///left most in case of a match
/// Sets the pivot point of the inputset
/// as the first element of the input set.
pub fn set_pivot(input_set: &mut Vec<Point2D>) -> &Vec<Point2D> {
    //panic if there are no elements in the input_set
    assert!(input_set.len() > 0);
    //set the vertex point to be the
    //first element of the inputset
    for i in 0..input_set.len() {
        if input_set[i].pick_left(&input_set[0]) == &input_set[i] {
            &input_set.swap(0, i);
        }
    }
    input_set
}

///Sorts the  elements of input set by polar
///angle in counter clockwise order around pivot point.
///(if more than one point has the same angle, remove all
///but the one that is farthest from pivot point)
pub fn sort_polar_angle_ccw(_input_set: &mut Vec<Point2D>) -> Vec<Point2D> {
    //TODO: implement the functionality
    Vec::new()
}
