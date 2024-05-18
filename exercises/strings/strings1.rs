// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
    println!("My old favorite color is {}", old_favorite_color());
}

fn current_favorite_color() -> String {
    "blue".to_string()
}

// &str: a string slice, a reference to a string, size is known at compile time
// 'static: the string is stored in the program's binary and is available for lifetime of the program
fn old_favorite_color() -> &'static str  {
    "red"
}