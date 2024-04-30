// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // variable bindings are immutable by default in Rust
    let mut x = 3; // set mutable to mutate it later
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
