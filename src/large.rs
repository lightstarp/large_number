mod new;
mod fix;
mod ord;
mod calculate;
mod display;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

#[derive(Clone, Copy, Debug)]
pub struct LargeNum {
    entry0: f32,
    entry1: i16,
    sign: Sign,
}

#[test]
fn add_check() {

    //  TEST1 / a + b
    let a = LargeNum::new(12345);
    let b = LargeNum::new(10029);
    assert_eq!(LargeNum::new(22374), a + b);

    //  TEST2 / 0 + n
    let a = LargeNum::new(0);
    let b = LargeNum::new("EEE100");
    assert_eq!(LargeNum::new("EEE100"), a + b);

}

#[test]
fn mul_check() {

    // TEST1 / a * b
    let a = LargeNum::new("E10");
    let b = LargeNum::new("E32");
    assert_eq!(LargeNum::new("E42"), a * b);

    // TEST2 / 0 * n
    let a = LargeNum::new(0);
    let b = LargeNum::new("EEE100");
    assert_eq!(LargeNum::new(0), a * b);

}

#[test]
fn pow_check() {

    // TEST1 / a ^ b
    let a = LargeNum::new("E7");
    let b = LargeNum::new(12);
    assert_eq!(LargeNum::new("E84"), a.pow(b));

    // TEST2 / 0 ^ n
    let a = LargeNum::new(0);
    let b = LargeNum::new("EE100");
    assert_eq!(LargeNum::new(0), a.pow(b));

    // TEST3 / 1 ^ n
    let a = LargeNum::new(1);
    let b = LargeNum::new("EE100");
    assert_eq!(LargeNum::new(1), a.pow(b));

    // TEST4 / n ^ 0
    let a = LargeNum::new("EE100");
    let b = LargeNum::new(0);
    assert_eq!(LargeNum::new(1), a.pow(b));

}

#[test]
fn ord_check() {

    // TEST1 / a == b
    let a = LargeNum::new("E10");
    let b = LargeNum::new("E10");
    assert!(a == b);

    // TEST2 / a > b
    let a = LargeNum::new("EE100");
    let b = LargeNum::new("E300");
    assert!(a > b);

    // TEST3 / a > b
    let a = LargeNum::new("E10#6");
    let b = LargeNum::new("E10#8");
    assert!(a < b);

}