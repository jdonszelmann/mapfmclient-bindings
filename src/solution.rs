use crate::coordinate::Coordinate;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Solution {
    pub paths: Vec<Vec<Coordinate>>,
}