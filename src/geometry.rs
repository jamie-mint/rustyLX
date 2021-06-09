
#[derive(Copy, Clone)]
pub struct CartesianPoint { // a float
    pub(crate) dimmensions: [f64; 3],
    num_dimmensions: usize,
}

impl CartesianPoint {
    pub fn new(dimmensions: [f64; 3]) -> CartesianPoint {
        CartesianPoint {
            dimmensions,
            num_dimmensions: dimmensions.len(),
        }
    }
}