use crate::lxmodel::lxmodel;
use crate::parameter::Parameter;

struct Pattern {
    model: lxmodel,
    temporal_scale: u32,
    parameters: Vec<Parameter>
}

fn random_write () {

}


#[cfg(test)]
mod tests {
    use cichlid::ColorRGB;

    #[test]
    fn test_color_enum() {
        assert_eq!(2 + 2, 4);
    }
}