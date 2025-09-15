use super::Lnum;

fn fix(value: f64) -> Lnum {
    match value {
        0.0.. => Lnum { sign:  1.0, tetra: 0, unit: value    },
        ..0.0 => Lnum { sign: -1.0, tetra: 0, unit: value    },
        _     => Lnum { sign:  1.0, tetra: 0, unit: f64::NAN }
    } 
}

pub trait Value { fn f(self) -> Result<Lnum, ()>; }
impl Value for f64 {
    fn f(self) -> Result<Lnum, ()> {
        Ok(fix(self))
    }
}
impl Value for i64 {
    fn f(self) -> Result<Lnum, ()> {
        Ok(fix(self as f64))
    }
}
impl Value for &str {
    fn f(self) -> Result<Lnum, ()> {
        lexer(self)
    }
}
impl Value for String {
    fn f(self) -> Result<Lnum, ()> {
        lexer(&self)
    }
}

fn lexer(s: &str) -> Result<Lnum, ()> {
    // 先頭に「-」がついてた場合、符号を負にして以降の文字を取得。無ければそのまま取得。
    let (s, sign) = match s.chars().next() {
        Some('-') => (s.get(1..).unwrap_or(""), -1.0),
        _         => (s, 1.0),
    };

    // 文中に「#」がついてた場合、左右に分割する。無ければデフォルト値。
    let (unit_str, tetra_str) = s.split_once('#').unwrap_or((s, "0"));

    // 先頭の「E」の数を計算。
    let mut count_e = 0;
    for (c, i) in unit_str.chars().zip(0..) { if c != 'E' { count_e = i; break } };

    // 「E」より後の文字列を取得。
    let (_, unit_str) = unit_str.split_at(count_e);

    // カーディナルとハイペリオンを数字に変換する。失敗した場合Noneを返す。
    let tetra = match tetra_str.parse::<u32>() { Ok(t) => t, Err(_) => return Err(()) };
    let unit  = match  unit_str.parse::<f64>() { Ok(t) => t, Err(_) => return Err(()) };

    // 正規化する。
    Ok(Lnum { sign, tetra: tetra + count_e as u32, unit })
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
    pub fn new<T: Value>(value: T) -> Lnum {
        value.f().unwrap()
    }

    /// new()と似ていますが、正常な引数ではない場合、Errを返します
    /// ```
    /// use large_number::Lnum;
    /// 
    /// fn example() {
    ///     println!("{}", Lnum::new("1.1"));        // Ok(1.10)
    ///     println!("{}", Lnum::new("E8"));         // Ok(E8.00)
    /// 
    ///     println!("{}", Lnum::new("1.1.1"));      // Err(())
    ///     println!("{}", Lnum::new("100#"));       // Err(())
    ///     println!("{}", Lnum::new("ABCDEFG"));    // Err(())
    /// }
    /// ```
    pub fn new_checked<T: Value>(value: T) -> Result<Lnum, ()> {
        value.f()
    }
}