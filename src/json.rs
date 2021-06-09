use serde_json::{Result, Value};
use crate::lxmodel::LXModel;
use crate::geometry::CartesianPoint;

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}


fn unpack_serde_value__to__f64_triple (v: &Value) -> [f64; 3] {
    [
        v[0].as_f64().unwrap(),
        v[1].as_f64().unwrap(),
        v[2].as_f64().unwrap(),
    ]
}

fn deserialize_cartesian_points_to_lxmodel(json_in_data: &str) -> Result<()>  {
    let v: Vec<Value> = serde_json::from_str(json_in_data)?;

    // Access parts of the data by indexing with square brackets.
    // println!("A Point {}", v[0]["point"]);

    let mut model: LXModel = LXModel::new(CartesianPoint::new(
        unpack_serde_value__to__f64_triple(&v[0]["point"])
    ));

    for p in &v {
        let point = CartesianPoint::new(
            unpack_serde_value__to__f64_triple(&p["point"])
        );

        model.add_point(point)
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::json::{untyped_example, deserialize_cartesian_points_to_lxmodel};
    use std::net::SocketAddr;
    use std::fs;
    use std::process::exit;
    use crate::lxmodel::LXModel;

    #[test]
    fn sanity() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test__untyped_example() ->  Result<(), Box<dyn std::error::Error + 'static>> {

        let filename = "data/freespace.json";
        let in_json_data = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        println!("{}", in_json_data);
        deserialize_cartesian_points_to_lxmodel(&in_json_data);

        Ok(())
    }

    #[test]
    fn load_cartesian_points() {

        // load_cartesian_points_to_model()

        // assert_eq!(lxmodel{}, lxmodel{})
    }
}