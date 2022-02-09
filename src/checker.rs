pub trait Checker {
    fn check(&self, number: i32) -> String;
}