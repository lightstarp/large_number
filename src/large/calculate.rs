use std::cmp::Ordering;
use super::Lnum;

pub(super) fn mia(lhs: f64, rhs: f64) -> f64 {
    match lhs - rhs {
        ..-16.0     => return rhs,
          ..0.0     => (10_f64.powf(rhs - lhs) + 1.0).log10() + lhs,
           16.0..   => return lhs,
            0.0..   => (10_f64.powf(lhs - rhs) + 1.0).log10() + rhs,
             _      => f64::NAN
    }
}

pub(super) fn mis(lhs: f64, rhs: f64) -> f64 {
    match lhs - rhs {
        ..-16.0     => return rhs,
          ..0.0     => (1.0 - 10_f64.powf(rhs - lhs)).log10() + lhs,
           16.0..   => return lhs,
            0.0..   => (1.0 - 10_f64.powf(lhs - rhs)).log10() + rhs,
             _      => f64::NAN
    }
}

pub(super) fn mia_s(lhs: f64, lhs_sign: f64, rhs: f64, rhs_sign: f64) -> f64 {
    match lhs - rhs {
        ..-16.0     => return rhs,
          ..0.0     => (lhs_sign * rhs_sign + 10_f64.powf(rhs - lhs)).log10() + lhs,
           16.0..   => return lhs,
            0.0..   => (lhs_sign * rhs_sign + 10_f64.powf(lhs - rhs)).log10() + rhs,
             _      => f64::NAN
    }
}

pub(super) fn mis_s(lhs: f64, lhs_sign: f64, rhs: f64, rhs_sign: f64) -> f64 {
    match lhs - rhs {
        ..-16.0     => return rhs,
          ..0.0     => (lhs_sign * rhs_sign - 10_f64.powf(rhs - lhs)).log10() + lhs,
           16.0..   => return lhs,
            0.0..   => (lhs_sign * rhs_sign - 10_f64.powf(lhs - rhs)).log10() + rhs,
             _      => f64::NAN
    }
}

pub(super) fn max(lhs: &Lnum, rhs: &Lnum) -> Lnum {
    match (lhs.tetra.cmp(&rhs.tetra), lhs.unit > rhs.unit) {
        (Ordering::Equal  , true ) => lhs.clone(),
        (Ordering::Equal  , false) => rhs.clone(),
        (Ordering::Greater, _    ) => lhs.clone(),
        (Ordering::Less   , _    ) => rhs.clone(),
    }
}

pub(super) fn up(value: &Lnum) -> Lnum {
    Lnum {
        sign:  value.sign,
        tetra: value.tetra + 1,
        unit:  value.unit.log10()
    }
}

pub(super) fn down(value: &Lnum) -> Lnum {
    Lnum {
        sign:  value.sign,
        tetra: value.tetra - 1,
        unit:  10_f64.powf(value.unit),
    }
}

pub(super) fn tetra_increment(value: &Lnum) -> Lnum {
    Lnum {
        sign:  value.sign,
        tetra: value.tetra + 1,
        unit:  value.unit,
    }
}

impl std::ops::Add for Lnum {
    type Output = Lnum;
    fn add(self, rhs: Lnum) -> Lnum {
        let (tetra, unit) = match (self.tetra, rhs.tetra) {
            (0, 0) => (0, self.unit * self.sign as f64 + rhs.unit * rhs.sign as f64),
            (0, 1) => (1, mia_s(self.unit.log10(), self.sign as f64, rhs.unit        , rhs.sign as f64)),
            (1, 0) => (1, mia_s(self.unit        , self.sign as f64, rhs.unit        , rhs.sign as f64)),
            (1, 1) => (1, mia_s(self.unit        , self.sign as f64, rhs.unit.log10(), rhs.sign as f64)),
            _      => return max(&self, &rhs),
        };

        if unit.is_infinite() {
            return up(&self) + up(&rhs)
        }

        match unit.partial_cmp(&0.0) {
            Some(Ordering::Greater) => Lnum { sign:  1.0, tetra, unit:  unit     },
            Some(Ordering::Equal  ) => Lnum { sign:  1.0, tetra, unit:  0.0      },
            Some(Ordering::Less   ) => Lnum { sign: -1.0, tetra, unit: -unit     },
            None                    => Lnum { sign:  1.0, tetra, unit:  f64::NAN },
        }
    }
}

impl std::ops::AddAssign for Lnum {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Lnum {
    type Output = Lnum;
    fn sub(self, rhs: Lnum) -> Lnum {
        let (tetra, unit) = match (self.tetra, rhs.tetra) {
            (0, 0) => (0, self.unit * self.sign as f64 - rhs.unit * rhs.sign as f64),
            (0, 1) => (1, mis_s(self.unit.log10(), self.sign as f64, rhs.unit        , rhs.sign as f64)),
            (1, 0) => (1, mis_s(self.unit        , self.sign as f64, rhs.unit        , rhs.sign as f64)),
            (1, 1) => (1, mis_s(self.unit        , self.sign as f64, rhs.unit.log10(), rhs.sign as f64)),
            _      => return max(&self, &rhs),
        };

        if unit.is_infinite() {
            return up(&self) - up(&rhs)
        }

        match unit.partial_cmp(&0.0) {
            Some(Ordering::Greater) => Lnum { sign:  1.0, tetra, unit:  unit     },
            Some(Ordering::Equal  ) => Lnum { sign:  1.0, tetra, unit:  0.0      },
            Some(Ordering::Less   ) => Lnum { sign: -1.0, tetra, unit: -unit     },
            None                    => Lnum { sign:  1.0, tetra, unit:  f64::NAN },
        }
    }
}

impl std::ops::SubAssign for Lnum {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for Lnum {
    type Output = Lnum;
    fn mul(self, rhs: Lnum) -> Lnum {
        let sign = self.sign * rhs.sign;
        let (tetra, unit) = match (self.tetra, rhs.tetra) {
            (0, 0) => (0, self.unit         * rhs.unit        ),
            (0, 1) => (1, self.unit.log10() + rhs.unit        ),
            (1, 0) => (1, self.unit         + rhs.unit.log10()),
            (1, 1) => (1, self.unit         + rhs.unit        ),
            (1, 2) => (2, mia(self.unit.log10(), rhs.unit        )),
            (2, 1) => (2, mia(self.unit        , rhs.unit        )),
            (2, 2) => (2, mia(self.unit        , rhs.unit.log10())),
            _      => return max(&self, &rhs),
        };

        if unit.is_infinite() {
            return up(&self) * up(&rhs)
        } else if unit.is_nan() {
            Lnum { sign:  1.0, tetra, unit:  f64::NAN }
        } else {
            Lnum { sign, tetra, unit }
        }
    }
}

impl std::ops::MulAssign for Lnum {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for Lnum {
    type Output = Lnum;
    fn div(self, rhs: Lnum) -> Lnum {
        let sign = self.sign * rhs.sign;
        let (tetra, unit) = match (self.tetra, rhs.tetra) {
            (0, 0) => (0, self.unit         / rhs.unit        ),
            (0, 1) => (1, self.unit.log10() - rhs.unit        ),
            (1, 0) => (1, self.unit         - rhs.unit.log10()),
            (1, 1) => (1, self.unit         - rhs.unit        ),
            (1, 2) => (2, mis(self.unit.log10(), rhs.unit        )),
            (2, 1) => (2, mis(self.unit        , rhs.unit        )),
            (2, 2) => (2, mis(self.unit        , rhs.unit.log10())),
            _      => return max(&self, &rhs),
        };

        if unit.is_infinite() {
            return up(&self) / up(&rhs)
        } else if unit.is_nan() {
            Lnum { sign:  1.0, tetra, unit:  f64::NAN }
        } else {
            Lnum { sign, tetra, unit }
        }
    }
}

impl std::ops::DivAssign for Lnum {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Lnum {
    pub fn pow(self: Lnum, rhs: Lnum) -> Lnum {
        if 0 != rhs.tetra && -1.0 == self.sign {
            return Lnum { sign:  1.0, tetra: 0, unit: f64::NAN };
        }
        let (tetra, unit) = match (self.tetra, rhs.tetra) {
            (0, 0) => (0, (self.unit * self.sign as f64).powf(rhs.unit * rhs.sign as f64)),
            (1, 0) => (1, self.unit         * rhs.unit         * rhs.sign as f64),
            (1, 1) => (2, self.unit.log10() + rhs.unit         * rhs.sign as f64),
            (2, 0) => (2, self.unit         + rhs.unit.log10() * rhs.sign as f64),
            (2, 1) => (2, self.unit         + rhs.unit         * rhs.sign as f64),
            (2, 2) => (3, mia(self.unit.log10(), rhs.unit         * rhs.sign as f64)),
            (3, 1) => (3, mia(self.unit        , rhs.unit.log10() * rhs.sign as f64)),
            (3, 2) => (3, mia(self.unit        , rhs.unit         * rhs.sign as f64)),
            _      => return max(&self, &tetra_increment(&rhs)),
        };

        if unit.is_infinite() {
            return up(&self).pow(rhs)
        } else if unit.is_nan() {
            return Lnum { sign:  1.0, tetra, unit: f64::NAN }
        } else if unit < 308.0 {

        }

        match unit.partial_cmp(&0.0) {
            Some(Ordering::Greater) => Lnum { sign:  1.0, tetra, unit:  unit     },
            Some(Ordering::Equal  ) => Lnum { sign:  1.0, tetra, unit:  0.0      },
            Some(Ordering::Less   ) => Lnum { sign: -1.0, tetra, unit: -unit     },
            None                    => Lnum { sign:  1.0, tetra, unit:  f64::NAN },
        }
    }

    pub fn log10(self: Lnum) -> Lnum {
        if self.sign == -1.0 {
            Lnum { sign: 1.0, tetra: 0, unit: f64::NAN }
        } else if self.tetra >= 1 {
            Lnum { sign: 1.0, tetra: self.tetra - 1, unit: self.unit }
        } else {
            Lnum { sign: 1.0, tetra: 0, unit: self.unit.log10() }
        }
    }
}