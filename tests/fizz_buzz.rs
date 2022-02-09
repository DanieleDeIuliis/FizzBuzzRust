use FizzBuzzRust;
use FizzBuzzRust::multiple_of_checker::MultipleOfChecker;
use FizzBuzzRust::not_multiple_of_checker::NotMultipleOfChecker;

#[test]
fn returns_fizz_with_a_multiple_of_three() {
    let result = FizzBuzzRust::FizzBuzz::new(
        &vec!(&MultipleOfChecker::new(3, "Fizz"),
              &MultipleOfChecker::new(5, "Buzz"),
              &NotMultipleOfChecker::new(&vec![3,5]))).fizz_buzz(3);

    assert_eq!("Fizz", result)
}

#[test]
fn returns_buzz_with_a_multiple_of_five() {
    let result = FizzBuzzRust::FizzBuzz::new(
        &vec!(&MultipleOfChecker::new(3, "Fizz"),
              &MultipleOfChecker::new(5, "Buzz"),
              &NotMultipleOfChecker::new(&vec![3,5]))).fizz_buzz(5);

    assert_eq!("Buzz", result)
}

#[test]
fn returns_fizzbuzz_with_a_multiple_of_both_five_and_three() {
    let result = FizzBuzzRust::FizzBuzz::new(
        &vec!(&MultipleOfChecker::new(3, "Fizz"),
              &MultipleOfChecker::new(5, "Buzz"),
              &NotMultipleOfChecker::new(&vec![3,5]))).fizz_buzz(15);

    assert_eq!("FizzBuzz", result)
}

#[test]
fn returns_the_initial_number_when_it_is_not_a_multiple_of_five_or_three() {
    let result = FizzBuzzRust::FizzBuzz::new(
        &vec!(&MultipleOfChecker::new(3, "Fizz"),
              &MultipleOfChecker::new(5, "Buzz"),
              &NotMultipleOfChecker::new(&vec![3,5]))).fizz_buzz(17);

    assert_eq!("17", result)
}