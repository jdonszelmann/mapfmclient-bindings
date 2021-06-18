use crate::coordinate::Coordinate;
use std::ops::{Deref, DerefMut};
use std::ops;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct MarkedCoordinate {
    #[serde(flatten)]
    coord: Coordinate,
    #[serde(rename="color")]
    colour: i64,
}

impl Deref for MarkedCoordinate {
    type Target = Coordinate;

    fn deref(&self) -> &Self::Target {
        &self.coord
    }
}

impl DerefMut for MarkedCoordinate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.coord
    }
}

impl_op_ex!(+ |a: &MarkedCoordinate, b: &Coordinate| -> MarkedCoordinate { MarkedCoordinate{ colour: a.colour, coord: Coordinate {x: a.x + b.x, y: a.y + b.y} }});
impl_op_ex!(- |a: &MarkedCoordinate, b: &Coordinate| -> MarkedCoordinate { MarkedCoordinate{ colour: a.colour, coord: Coordinate {x: a.x - b.x, y: a.y - b.y} }});
impl_op_ex!(+ |a: &Coordinate, b: &MarkedCoordinate| -> MarkedCoordinate { MarkedCoordinate{ colour: b.colour, coord: Coordinate {x: a.x + b.x, y: a.y + b.y} }});
impl_op_ex!(- |a: &Coordinate, b: &MarkedCoordinate| -> MarkedCoordinate { MarkedCoordinate{ colour: b.colour, coord: Coordinate {x: a.x - b.x, y: a.y - b.y} }});
