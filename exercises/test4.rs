// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

macro_rules! my_macro {
    ($val:expr) => {
        // solution 1, create a long expression
        // {
        //     let mut ret = "Hello ".to_string();
        //     ret.push_str(&$name);
        //     ret
        // }

        // format the output
        format!("Hello {}", $val)
    }
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
