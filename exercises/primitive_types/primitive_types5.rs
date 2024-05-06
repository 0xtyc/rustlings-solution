// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
