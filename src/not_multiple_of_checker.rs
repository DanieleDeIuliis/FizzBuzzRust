use std::borrow::Borrow;
use crate::Checker;

pub struct NotMultipleOfChecker<'a> {
    numbers: &'a Vec<i32>
}

impl<'a> NotMultipleOfChecker<'a> {
    pub fn new(numbers: &'a Vec<i32>) -> Self {
        NotMultipleOfChecker { numbers }
    }
}

impl<'a> Checker for NotMultipleOfChecker<'a> {
    fn check(&self, number: i32) -> String {
        if self.numbers.iter().any(|base| number % base == 0 ) {
            String::from("")
        } else {
         number.to_string()
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn return_empty_when_the_number_is_a_multiple_of_a_base() {
        assert_eq!(String::from(""), NotMultipleOfChecker::new(&vec![3,5]).check(15))
    }

    #[test]
    fn return_the_number_when_it_is_not_a_multiple_of_any_base() {
        assert_eq!(String::from("200"), NotMultipleOfChecker::new(&vec![7]).check(200))
    }
}