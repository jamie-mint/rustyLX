use crate::geometry::CartesianPoint;
use serde_json::Value;

pub struct LXModel {
    points: Vec<CartesianPoint>,
    num_points: usize,
}

impl LXModel {
    pub fn new(first_point: CartesianPoint) -> LXModel {
        LXModel {
            points: vec![first_point],
            num_points: first_point.dimmensions.len(),
        }
    }

    pub fn add_point(&mut self, point: CartesianPoint) {

        self.points.push(point)
    }
}

impl LXModel {
    pub fn ingest_points() {

    }
}