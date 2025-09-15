mod calculate;
mod cmp;
mod format;
mod new;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lnum {
    sign: f32,
    tetra: u32,
    unit: f64,
}

#[cfg(test)]
use std::time::Instant;

#[cfg(test)]
fn time_check<T>(default: T, f: impl Fn(T) -> T, name: &str) {
    let start = Instant::now();
    
    let mut x = default;
    for _ in 0..=10_000_000 {
        x = f(x);
    }
    
    let end = start.elapsed();
    println!("{name:>2}:{}.{:06}second", end.as_secs(), end.subsec_nanos() / 1_000);
}

#[test]
pub fn test() {
    let x = Lnum::new(10.0);
    println!("--- Processing speed verification through 10,000,000 tests ---");
    println!("");
    time_check(Lnum::new(30.0), |i| {
        i + x
    }, "+");
    time_check(Lnum::new(30.0), |i| {
        i - x
    }, "-");
    time_check(Lnum::new(30.0), |i| {
        i * x
    }, "*");
    time_check(Lnum::new(30.0), |i| {
        i / x
    }, "/");
    time_check(Lnum::new(30.0), |i| {
        i.pow(x)
    }, "^");
    println!("");
    println!("--- Result tests ---");
    // let mut i1 = Lnum::new(0.0);
    // for _ in 0..120 {
    //     let i0 = i1;
    //     i1 = i0 + Lnum::new(1.0);
    //     let result = i1 / i0;
    //     println!("{:13} -> x{:13}", i1, result)
    // }

    for i in 0..=300 {
        let i = i * 10;
        let x0 = Lnum::new(i);
        let x0 = x0;
        let x1 = x0.pow(Lnum::new(42.0));
        let x2 = Lnum::new(1.05).pow(
            Lnum::new(1.05).pow(
                Lnum::new(1.05).pow(
                    x0
                )
            )
        );
        let b  = x0 >= Lnum::new("EEE50");
        println!("x: {:13} 42^x: {:13} 1.05^(1.05^(1.05^x)): {:13} bool: {:13}", x0, x1, x2, b)
    }
}