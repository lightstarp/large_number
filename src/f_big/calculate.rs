use super::{FBig, Sign, fix::fix};

#[inline]
fn calm1(l: f32, r: f32) -> f32 {
    l + r
}
#[inline]
fn cal0(l: f32, r: f32) -> f32 {
    match l - r {
        -15.0..0.0 => (10_f32.powf(l - r) + 1.0).log10() + r,
        0.0..15.0  => (10_f32.powf(r - l) + 1.0).log10() + l,
        ..0.0      => r,
        0.0..      => l,
        _          => unreachable!(),
    }
}
#[inline]
fn cal1(l: f32, r: f32) -> f32 {
    l + r
}
#[inline]
fn cal2(l: f32, r: f32) -> f32 {
    l * r
}
#[inline]
fn cal3(l: f32, r: f32) -> f32 {
    l.powf(r.log10())
}

fn cal(l: (f32, i16), r: (f32, i16)) -> (f32, i16) {
    let x = match l.1 - r.1 {
        ..=-3   => return r,
           -2   => (l.0.log10().log10(), r.0                , r.1),
           -1   => (l.0.log10()        , r.0                , r.1),
            0   => (l.0                , r.0                , l.1),
            1   => (l.0                , r.0.log10()        , l.1),
            2   => (l.0                , r.0.log10().log10(), l.1),
            3.. => return l,
    };
    let o = match x.2 {
        -2 => ( cal3(x.0, x.1), x.2),
        -1 => ( cal2(x.0, x.1), x.2),
         0 => ( cal1(x.0, x.1), x.2),
         1 => ( cal0(x.0, x.1), x.2),
         2 => (calm1(x.0, x.1), x.2),
         _ => { if x.0 > x.1 { l } else { r } }
    };
    fix(o)
}

impl std::ops::Add for FBig {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let x = cal((self.entry0, self.entry1), (rhs.entry0, rhs.entry1));
        FBig { entry0: x.0, entry1: x.1, s: Sign::Plus }
    }
}

impl std::ops::Mul for FBig {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        if self == FBig::new(0) { return FBig::new(0) }
        if rhs  == FBig::new(0) { return FBig::new(0) }
        let x = cal((self.entry0, self.entry1 - 1), (rhs.entry0, rhs.entry1 - 1));
        FBig { entry0: x.0, entry1: x.1 + 1, s: Sign::Plus }
    }
}

impl FBig {
    pub fn pow(self, rhs: Self) -> Self {
        if self == FBig::new(1) { return FBig::new(1) }
        if self == FBig::new(0) { return FBig::new(0) }
        if rhs  == FBig::new(0) { return FBig::new(1) }
        let x = cal((self.entry0, self.entry1 - 2), (rhs.entry0, rhs.entry1 - 1));
        FBig { entry0: x.0, entry1: x.1 + 2, s: Sign::Plus }
    }
}