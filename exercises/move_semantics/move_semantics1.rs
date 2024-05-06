// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0); // vec1 is still immutable

    assert_eq!(vec1, vec![22, 44, 66, 88]);
    // here we can't use vec0 anymore because its ownership was moved after calling fill_vec

    let mut vec2 = vec![11, 33, 55];
    fill_vec2(&mut vec2); // vec2 is mutable
    assert_eq!(vec2, vec![11, 33, 55, 77]);
    vec2.push(99); // we can still push to vec2 because it's mutable
    assert_eq!(vec2, vec![11, 33, 55, 77, 99]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // we can push to a vec only when it's mutable
    let mut vec = vec; 

    vec.push(88);

    vec
}

//  passing a reference to a function allows the function to borrow the variable without taking ownership.
fn fill_vec2(vec: &mut Vec<i32>) {
    vec.push(77);
}