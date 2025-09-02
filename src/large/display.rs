use super::LargeNum;

impl std::fmt::Display for LargeNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        match self.entry1 {
            -2   => { s = format!("{:.1}", self.entry0.log10().log10()); }
            -1   => { s = format!("{:.1}", self.entry0.log10()); }
            0..3 => {
                for _ in 0..self.entry1 {
                    s.push('E')
                }
                s.push_str(&format!("{:.2}", self.entry0));
            }
            _    => { s = format!("E{:08.1}#{}", self.entry0, self.entry1) }
        };

        
        if let Some(a) = f.align() {
            match a {
                std::fmt::Alignment::Left   => write!(f, "{:<12}", s),
                std::fmt::Alignment::Right  => write!(f, "{:>12}", s),
                std::fmt::Alignment::Center => write!(f, "{:^12}", s),
            }
        } else {
            write!(f, "{:12}", s)
        }
    }
}