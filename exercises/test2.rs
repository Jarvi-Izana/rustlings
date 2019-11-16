// test2.rs
// This is a test for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `Strings`, some are `&strs`. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string_slice(&String::from("abc")[0..1]);

    string("red".to_string());
    string(String::from("hi"));

    // to owned is preferred since
    // it has better documentation purpose
    string("rust is fun!".to_owned());

    // oh, really?? it has type deduction???
    string_slice("nice weather".into());
    string("nice weather".into());

    // no write just reference
    string_slice("  hello there ".trim());

    // if you need to write change to string
    // it should return an owned String type
    string(format!("Interpolation {}", "Station"));
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

