use super::{Lnum, Sign, fix::fix};

pub trait Value { fn f(self) -> Option<Lnum>; }
impl Value for f32 {
    fn f(self) -> Option<Lnum> {
        let x = fix((self, 0));
        Some(Lnum { entry0: x.0, entry1: x.1, sign: Sign::Plus })
    }
}
impl Value for i32 {
    fn f(self) -> Option<Lnum> {
        let x = fix((self as f32, 0));
        Some(Lnum { entry0: x.0, entry1: x.1, sign: Sign::Plus })
    }
}
impl Value for &str {
    fn f(self) -> Option<Lnum> {
        lexer(self)
    }
}
impl Value for String {
    fn f(self) -> Option<Lnum> {
        lexer(&self)
    }
}

fn lexer(s: &str) -> Option<Lnum> {
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
    Some(Lnum { entry0, entry1, sign })
}

impl Lnum {
    /// Lnum形式の数値を新しく作成します
    /// 
    /// # Example
    /// ```
    /// use large_number::Lnum;
    /// 
    /// fn example() {
    ///     let x = Lnum::new(1729);
    ///     let y = Lnum::new(1111);
    ///     
    ///     println!("{}", x); // 1729.00
    ///     println!("{}", y); // 1111.00
    ///     println!("{}", x + y); // 2840.00
    /// }
    /// ```
    /// 
    /// 関数の引数は[`i32`],[`f32`],[`str`],[`String`]に対応しており、
    /// それぞれ同じ結果が得られます。
    /// 
    /// ```
    /// use large_number::Lnum;
    /// 
    /// fn example() {
    ///     let a = Lnum::new(42);
    ///     let b = Lnum::new(42.0);
    ///     let c = Lnum::new("42");
    ///     let d = Lnum::new("42".to_string());
    ///     
    ///     println!("{}", a == b && b == c && c == d); // true
    /// }
    /// ```
    /// 
    /// [`str`],[`String`]は大きい数値を記述するときに有効的に使えます。
    /// 
    /// ```
    /// use large_number::Lnum;
    /// 
    /// fn example() {
    ///     println!("{}", Lnum::new("12345.6")); // 12345.60
    ///     println!("{}", Lnum::new("E8")); // E8.00
    ///     println!("{}", Lnum::new("EE3")); // E1000.00
    ///     println!("{}", Lnum::new("E10#100")); // E10.00#100
    /// }
    /// ```
    pub fn new<V: Value>( v: V ) -> Lnum {
        v.f().unwrap()
    }

    pub fn new_checked<V: Value>( v: V ) -> Option<Lnum> {
        v.f()
    }
}