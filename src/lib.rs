use crate::checker::Checker;

pub mod multiple_of_checker;
mod checker;
pub mod not_multiple_of_checker;

pub struct FizzBuzz<'a> {
    checkers: &'a Vec<&'a dyn Checker>
}

impl<'a> FizzBuzz<'a> {

    pub fn new(checkers: &'a Vec<&'a dyn Checker>) -> Self {
        FizzBuzz { checkers }
    }

    pub fn fizz_buzz(&self, number: i32) -> String {
        self.checkers.iter()
            .map(|checker| checker.check(number))
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod test {
    use crate::multiple_of_checker::MultipleOfChecker;
    use crate::not_multiple_of_checker::NotMultipleOfChecker;
    use super::*;

    #[test]
    fn returns_fizz_with_a_multiple_of_three() {
        assert_eq!("Fizz", FizzBuzz::new(
            &vec!(&MultipleOfChecker::new(3, "Fizz"),
                     &MultipleOfChecker::new(5, "Buzz"),
                     &NotMultipleOfChecker::new(&vec![3,5]))).fizz_buzz(3))
    }

    #[test]
    fn returns_buzz_with_a_multiple_of_five() {
        assert_eq!("Buzz", FizzBuzz::new(
            &vec!(&MultipleOfChecker::new(3, "Fizz"),
                  &MultipleOfChecker::new(5, "Buzz"),
                  &NotMultipleOfChecker::new(&vec![3,5]))).fizz_buzz(5))
    }

    #[test]
    fn returns_fizzbuzz_with_a_multiple_of_both_five_and_three() {
        assert_eq!("FizzBuzz", FizzBuzz::new(
            &vec!(&MultipleOfChecker::new(3, "Fizz"),
                  &MultipleOfChecker::new(5, "Buzz"),
                  &NotMultipleOfChecker::new(&vec![3,5]))).fizz_buzz(15))
    }

    #[test]
    fn returns_the_initial_number_when_it_is_not_a_multiple_of_five_or_three() {
        assert_eq!("8", FizzBuzz::new(
            &vec!(&MultipleOfChecker::new(3, "Fizz"),
                  &MultipleOfChecker::new(5, "Buzz"),
                  &NotMultipleOfChecker::new(&vec![3,5]))).fizz_buzz(8))
    }
}