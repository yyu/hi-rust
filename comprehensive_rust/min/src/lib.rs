use std::cmp::Ordering;

fn mymin<T: Ord>(x: T, y: T) -> T {
    x.min(y)
}

#[test]
fn integers() {
    assert_eq!(mymin(0, 10), 0);
    assert_eq!(mymin(500, 123), 123);
}

#[test]
fn chars() {
    assert_eq!(mymin('a', 'z'), 'a');
    assert_eq!(mymin('7', '1'), '1');
}

#[test]
fn strings() {
    assert_eq!(mymin("hello", "goodbye"), "goodbye");
    assert_eq!(mymin("bat", "armadillo"), "armadillo");
}
