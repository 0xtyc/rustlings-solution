// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let mut x = 100;
    let y = &mut x; // creates a mutable reference y to x. y points to the same memory location as x and can be used to modify x
    *y += 100; // dereference y and add 100 to the value it points to
    assert_eq!(*y, 200);
    let z = &mut x; 
    // *y can't be used anymore 
    // because we can't create a second reference between the creation of first mutable reference and its usage
    *z += 1000;
    assert_eq!(*z, 1200);
    assert_eq!(x, 1200);
}
