use large_number::Lnum;

fn main() {
    let x = Lnum::new(1.0);
    time_check(Lnum::new(27.0), |i| {
        i + x
    }, "+");
    time_check(Lnum::new(27.0), |i| {
        i * x
    }, "*");
}

use std::time::Instant;

fn time_check<T>(init: T, f: impl Fn(T) -> T, name: &str) {
    let start = Instant::now();
    
    let mut x = init;
    for _ in 0..=10 {
        for _ in 0..=1_000_000 {
            x = f(x);
        }
    }
    
    let end = start.elapsed();
    println!("{name:>12}:{}.{:06}second", end.as_secs(), end.subsec_nanos() / 1_000);
}

#[test]
fn test() {
    let start = Instant::now();
    
    for _ in 0..=1 {
        for i in 0..=100_000_000 {
            let _ = i as f32;
        }
    }
    
    let end = start.elapsed();
    println!("{}.{:06}second", end.as_secs(), end.subsec_nanos() / 1_000);
}