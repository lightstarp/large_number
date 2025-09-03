use crate::Lnum;

#[test]
fn add_check() {

    //  a + b
    let a = Lnum::new(12345);
    let b = Lnum::new(10029);
    assert_eq!(Lnum::new(22374), a + b);

    //  0 + n
    let a = Lnum::new(0);
    let b = Lnum::new("EEE100");
    assert_eq!(Lnum::new("EEE100"), a + b);

}

#[test]
fn mul_check() {

    //  a * b
    let a = Lnum::new("E10");
    let b = Lnum::new("E32");
    assert_eq!(Lnum::new("E42"), a * b);

    //  0 * n
    let a = Lnum::new(0);
    let b = Lnum::new("EEE100");
    assert_eq!(Lnum::new(0), a * b);

}

#[test]
fn pow_check() {

    //  a ^ b
    let a = Lnum::new("E7");
    let b = Lnum::new(12);
    assert_eq!(Lnum::new("E84"), a.pow(b));

    // 0 ^ n
    let a = Lnum::new(0);
    let b = Lnum::new("EE100");
    assert_eq!(Lnum::new(0), a.pow(b));

    //  1 ^ n
    let a = Lnum::new(1);
    let b = Lnum::new("EE100");
    assert_eq!(Lnum::new(1), a.pow(b));

    //  n ^ 0
    let a = Lnum::new("EE100");
    let b = Lnum::new(0);
    assert_eq!(Lnum::new(1), a.pow(b));

}

#[test]
fn ord_check() {

    //  a == b
    let a = Lnum::new("E10");
    let b = Lnum::new("E10");
    assert!(a == b);

    //  a > b
    let a = Lnum::new("EE100");
    let b = Lnum::new("E300");
    assert!(a > b);

    //  a > b
    let a = Lnum::new("E10#6");
    let b = Lnum::new("E10#8");
    assert!(a < b);

}