use edtest::{fixture, test};

#[fixture]
fn base() -> u32 {
    40
}

#[fixture]
fn answer(base: u32) -> u32 {
    base + 2
}

#[test]
fn uses_fixture(answer: u32) {
    assert_eq!(answer, 42);
}
