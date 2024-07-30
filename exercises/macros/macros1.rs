// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // called macro with `!` to distinguish it from a function
    my_macro!();
}
