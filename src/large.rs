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
pub struct Lnum {
    entry0: f32,
    entry1: i16,
    sign: Sign,
}