// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.


fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    // when you iterate over a vector with a loop
    // you get references to the elements of the vector, not the elements themselves.
    for element in v.iter_mut() {
        *element = *element * 2; // the dereference operator (*) is used to get the value of the reference
        //storing the result back into the memory location that element points to.
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        *element * 2 // or "element * 2" (rust will automatically dereference it &i32 -> i32 but not &mut i32)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
