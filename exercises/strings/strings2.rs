// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
    let another_word = "black";
    if is_a_color_word(another_word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

// with the &str type, the function is_a_color_word can accept both &String and &str types
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
