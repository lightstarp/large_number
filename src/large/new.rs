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
    /// // 関数の引数に値を入れることでLnumを作成することができます。
    /// fn example1() {
    ///     let x = Lnum::new(1729);
    ///     let y = Lnum::new(1111);
    ///     
    ///     println!("{}", x);      // 1729.00
    ///     println!("{}", y);      // 1111.00
    ///     println!("{}", x + y);  // 2840.00
    /// }
    /// 
    /// // 関数の引数はi32, f32, &str, Stringなどに対応しています。
    /// fn example2() {
    ///     let a = Lnum::new(42);                  //i32
    ///     let b = Lnum::new(42.0);                //f32
    ///     let c = Lnum::new("42");                //&str
    ///     let d = Lnum::new("42".to_string());    //String
    ///     
    ///     println!("{}", a == b && b == c && c == d); // true
    /// }
    /// 
    /// // &str, Stringは大きい数値を記述するときに有効的に使えます。
    /// fn example3() {
    ///     println!("{}", Lnum::new("12345.6"));   // 12345.60
    ///     println!("{}", Lnum::new("E8"));        // E8.00
    ///     println!("{}", Lnum::new("EE3"));       // E1000.00
    ///     println!("{}", Lnum::new("E10#100"));   // E010.0#100
    /// }
    /// ```
    /// 
    /// # Panic
    /// 不正な文字列を引数に入れるとPanicを引き起こします
    /// ```
    /// use large_number::Lnum;
    /// 
    /// fn example() {
    ///     println!("{}", Lnum::new("1.1"));           // 1.10
    ///     println!("{}", Lnum::new("E8"));            // E8.00
    /// 
    ///     // println!("{}", Lnum::new("1.1.1"));      // Panic!
    ///     // println!("{}", Lnum::new("100#"));       // Panic!
    ///     // println!("{}", Lnum::new("ABCDEFG"));    // Panic!
    /// }
    /// ```
    pub fn new<V: Value>( v: V ) -> Lnum {
        v.f().unwrap()
    }

    /// 正常な値なのかを、Option型で返します
    /// ```
    /// use large_number::Lnum;
    /// 
    /// fn example() {
    ///     println!("{}", Lnum::new("1.1"));        // Some(1.10)
    ///     println!("{}", Lnum::new("E8"));         // Some(E8.00)
    /// 
    ///     println!("{}", Lnum::new("1.1.1"));      // None
    ///     println!("{}", Lnum::new("100#"));       // None
    ///     println!("{}", Lnum::new("ABCDEFG"));    // None
    /// }
    /// ```
    pub fn new_checked<V: Value>( v: V ) -> Option<Lnum> {
        v.f()
    }
}