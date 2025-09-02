#[inline]
fn up(x: (f32, i16)) -> (f32, i16) {
    (x.0.log10(), x.1 + 1)
}

#[inline]
fn down(x: (f32, i16)) -> (f32, i16) {
    (10_f32.powf(x.0), x.1 - 1)
}

pub fn fix(x: (f32, i16)) -> (f32, i16) {
    match x.0 {
        1e+6.. => fix(up(x)),
        ..6.0  => fix(down(x)),
        _         => x,
    }
}

//pub fn fix_struct(x: FBig) -> FBig {
//    let y = fix((x.f, x.h));
//    FBig { f: y.0, h: y.1, s: x.s }
//}