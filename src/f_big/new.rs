use super::{FBig, Sign, fix::fix};

pub trait Value { fn f(self) -> Option<FBig>; }
impl Value for f32 {
    fn f(self) -> Option<FBig> {
        let x = fix((self, 0));
        Some(FBig { entry0: x.0, entry1: x.1, s: Sign::Plus })
    }
}
impl Value for i32 {
    fn f(self) -> Option<FBig> {
        let x = fix((self as f32, 0));
        Some(FBig { entry0: x.0, entry1: x.1, s: Sign::Plus })
    }
}
impl Value for &str {
    fn f(self) -> Option<FBig> {
        lexer(self)
    }
}
impl Value for String {
    fn f(self) -> Option<FBig> {
        lexer(&self)
    }
}

fn lexer(s: &str) -> Option<FBig> {
    // 先頭に「-」がついてた場合、符号を負にして以降の文字を取得。無ければそのまま取得。
    let (s, sign) = match s.chars().next() {
        Some('-') => (s.get(1..).unwrap_or(""), Sign::Minus),
        _         => (s, Sign::Plus),
    };

    // 文中に「#」がついてた場合、左右に分割する。無ければデフォルト値。
    let (entry0_str, entry1_str) = s.split_once('#').unwrap_or((s, "1"));

    // 先頭の「E」の数を計算。
    let mut count_e = 0;
    for (c, i) in entry0_str.chars().zip(0..) { if c != 'E' { count_e = i; break } };

    // 「E」より後の文字列を取得。
    let (_, entry0_str) = entry0_str.split_at(count_e);

    // カーディナルとハイペリオンを数字に変換する。失敗した場合Noneを返す。
    let entry1 = match entry1_str.parse::<i16>() { Ok(t) => t, Err(_) => return None };
    let entry0 = match entry0_str.parse::<f32>() { Ok(t) => t, Err(_) => return None };

    // 正規化する。
    let (entry0, entry1) = fix((entry0, entry1 * count_e as i16));
    Some(FBig { entry0, entry1, s: sign })
}

impl FBig {
    pub fn new<V: Value>( v: V ) -> FBig {
        v.f().unwrap()
    }

    pub fn new_checked<V: Value>( v: V ) -> Option<FBig> {
        v.f()
    }
}