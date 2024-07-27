// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar {
    fn append_bar_1(self) -> Self;
    fn append_bar_2(self) -> Self;
}

impl AppendBar for Vec<String> {
    // the implementation can allow the method to be called on mutable self
    fn append_bar_1(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
    fn append_bar_2(self) -> Self {
        let mut new_vec = self.clone();
        new_vec.push(String::from("Bar"));
        new_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar_1();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
        let vec = vec![String::from("Foo")];
        vec.append_bar_2(); // the method takes the ownership of the vector
        // the vec is not usable after the append_bar_1 or 2 method is called
    }
}
