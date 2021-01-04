fn answer() -> i8 { 42 }

pub fn main() {
    answer();
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}
