use super::{
    Lnum,
    calculate::down,
    calculate::up,
};

const CORRECTION: f64 = 0.000000001;

// struct LnumFmt<'a>{
//     notation: Notation,
//     precision: usize,
//     main: &'a Lnum,
// }
// 
// enum Notation {
//     Exponential,
//     Generic,
// }

impl Lnum {
    pub fn scientific(&self, precision: usize) -> String {
        let value = if self.tetra != 0 && self.unit < 6.0 {
            down(self)
        } else if self.unit < 1e6 {
            self.clone()
        } else {
            up(self)
        };
        let result = match value.tetra {
            0 => {
                if self.unit < 100.0 {
                    let unit = value.unit + CORRECTION;
                    format!("{unit:.*}", precision)
                } else {
                    let unit = (value.unit + CORRECTION).floor();
                    format!("{unit:.0}")
                }
            }
            1..3 => {
                let mut str = String::new();
                for _ in 1..value.tetra {
                    str.push_str("1.00e");
                }
                let e1 = (value.unit + CORRECTION).floor();
                let e0 = 10_f64.powf(value.unit - e1);
                format!("{str}{e0:.2}e{e1:.0}")
            }
            3.. => {
                let mut unit = value.unit;
                let mut tetra = value.tetra;
                loop {
                    if unit < 10.0 { break }
                    unit = unit.log10();
                    tetra += 1;
                }
                format!("E{unit:.4}#{tetra:.0}")
            }
        };
        let sign = match self.sign {
            1.0 => "",
            _   => "-",
        };
        format!("{:16}", format!("{sign}{result}"))
    }

    // fn fmt<'a>(&'a self) -> LnumFmt<'a> {
    //     LnumFmt {
    //         notation: Notation::Exponential,
    //         precision: 3,
    //         main: self
    //     }
    // }

    pub fn exponential(&self, precision: usize) -> String {
        if !self.unit.is_normal() {
            if self.unit == 0.0 {
                return format!("{:.*}", precision,  CORRECTION)
            } else if self.unit == f64::INFINITY {
                return "inf".to_string()
            } else if self.unit == f64::NEG_INFINITY {
                return "-inf".to_string()
            } else {
                return "NaN".to_string()
            }
        }
        let value = if self.unit >= 1e6 {
            up(self)
        } else if self.tetra != 0 && self.unit < 6.0 {
            down(self)
        } else {
            *self
        };
        if value.tetra <= 3 {
            let mut s = String::new();
            for _ in 0..value.tetra {
                s.push('E');
            };
            if value.unit < 1000.0 {
                format!("{}{:.*}", s, precision, value.unit)
            } else {
                format!("{}{:.0}", s, value.unit)
            }
        } else {
            let (unit, tetra) = if self.unit >= 10.0 {
                (self.unit.log10(), self.tetra + 1)
            } else {
                (self.unit, self.tetra)
            };
            format!("E{:.4}#{:.0}", unit, tetra)
        }
    }
}

// impl<'a> LnumFmt<'a> {
//     pub fn precision(mut self, precision: usize) -> Self {
//         self.precision = precision;
//         self
//     }
//     pub fn notation(mut self, notation: Notation) -> Self {
//         self.notation = notation;
//         self
//     }
// }

impl std::fmt::Display for Lnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(3);
        if let Some(width) = f.width() {
            write!(f, "{:width$}", self.exponential(precision))
        } else {
            write!(f, "{}", self.exponential(precision))
        }
    }
}