// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // `if let` allows you to match the patter and execute block of code
        //  for this pattern matched, word will be available in the execution block
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;
   
        // optional_integers has type Vec<Option<i8>> 
        // .pop() returns an Option<T> so we need to match it with Some(Some(integer))
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
        assert_eq!(optional_integers.len(), 0);
    }

    #[test]
    fn basic_option() {
        let range = 10;
        let mut integers: Vec<i8> = vec![];

        for i in 1..(range + 1) {
            integers.push(i);
        }

        let mut cursor = range;
   
        // the `pop` method returns an Option<T> so we need to match it with Some(integer)
        while let Some(integer) = integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
        assert_eq!(integers.len(), 0);
    }
}
