// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);

    // the ownership of data has been moved to string_uppercase
    // so we can't use it anymore
}

// Should not take ownership
// it takes a reference to a string, which means it borrows the string temporarily during the function call
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// Rust deallocate the memory of the string after the function call
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
