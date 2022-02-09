use crate::Checker;

pub struct MultipleOfChecker {
    base: i32,
    output: &'static str
}

impl MultipleOfChecker {
    pub fn new(number: i32, value: &'static str) -> Self {
        MultipleOfChecker { base: number, output: value}
    }
}

impl Checker for MultipleOfChecker {
    fn check(&self, number: i32) -> String {
        if number % self.base == 0 { self.output.to_string() } else { String::from("") }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn return_fizz_when_the_number_is_a_multiple_of_three() {
        assert_eq!(String::from("Fizz"), MultipleOfChecker::new(3, "Fizz").check(69))
    }

    #[test]
    fn return_empty_string_when_the_number_is_a_multiple_of_three() {
        assert_eq!(String::from(""), MultipleOfChecker::new(3, "Fizz").check(200))
    }
}