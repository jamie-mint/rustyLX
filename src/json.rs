use serde_json::{Result, Value};

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

fn deserialize_cartesian_points(json_in_data: &str) -> Result<()>  {
    let v: Value = serde_json::from_str(json_in_data)?;

    // Access parts of the data by indexing with square brackets.
    println!("A Point {}", v[0]["point"]);

    // Load points into lxmodel.


    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::json::{untyped_example, deserialize_cartesian_points};
    use std::net::SocketAddr;
    use std::fs;
    use std::process::exit;
    use crate::lxmodel::lxmodel;

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
        deserialize_cartesian_points(&in_json_data);

        Ok(())
    }

    #[test]
    fn load_cartesian_points() {

        // load_cartesian_points_to_model()

        // assert_eq!(lxmodel{}, lxmodel{})
    }
}