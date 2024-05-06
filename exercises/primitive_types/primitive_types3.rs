// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let a = ["A"; 101]; // short hand for creating an array of 101 elements with the value "A"
    let a = "A".repeat(101); // long string can also be len >= 100 

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed"); // this will terminate the programe
    }
}
