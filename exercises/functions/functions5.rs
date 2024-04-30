// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num // or return num * num; 
    // the last expression is automatically returned in rust 
    // but after adding a semicolon, it becomes a statement and does not return a value
}
