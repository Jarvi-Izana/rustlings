// structs1.rs
// Address all the TODOs to make the tests pass!



struct ColorClassicStruct {
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String);

// struct without any fields
#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct {
            name: "green".to_string(),
            hex: "#00FF00".to_string(),
        };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // Instantiate a tuple struct!
        // For more fun, use the field initialization shorthand.
        let green = ("green".to_string(), "#00FF00".to_string());

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // Instantiate a unit struct!
        // without curly brace is also okay!
        // let unit_struct = UnitStruct;
        let unit_struct = UnitStruct{};

        // debug print by trait Debug
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
