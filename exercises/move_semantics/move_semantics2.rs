// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    // when an argument is passed to a function and it's not explicitly returned
    // you can't use the original variable anymore.
    let mut vec1 = fill_vec(vec0.clone()); // clone the vec0, so that it can be used later 
    let vec2 = fill_vec2(&vec0);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
    assert_eq!(vec2, vec![22, 44, 66, 88]);

    // three vectors are created and stored in memory
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn fill_vec2(vec: & Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();
    vec.push(88);
    vec
}