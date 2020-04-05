use std::ops::*;

pub type Vi = isize;
pub type Vf = f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct V2i(pub Vi, pub Vi);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct V2f(pub Vf, pub Vf);

impl V2i {
    pub fn l1(self) -> Vi { self.0.abs() + self.1.abs() }
    pub fn linf(self) -> Vi { self.0.abs().min(self.1.abs()) }
    pub fn swap(self) -> V2i { V2i(self.1, self.0) }
    pub fn abs(self) -> V2i { V2i(self.0.abs(), self.1.abs()) }
    pub fn div_euclid(self, other: V2i) -> V2i { V2i(self.0.div_euclid(other.0), self.1.div_euclid(other.1)) }
    pub fn rem_euclid(self, other: V2i) -> V2i { V2i(self.0.rem_euclid(other.0), self.1.rem_euclid(other.1)) }
    pub fn is_q1(self) -> bool { self.0 >= 0 && self.1 >= 0 }
    pub fn is_strict_q1(self) -> bool { self.0 > 0 && self.1 > 0 }
}

impl V2f {
    pub fn l1(self) -> Vf { self.0.abs() + self.1.abs() }
    pub fn l2(self) -> Vf { (self.0 * self.0 + self.1 * self.1).sqrt() }
    pub fn linf(self) -> Vf { self.0.abs().min(self.1.abs()) }
    pub fn swap(self) -> V2f { V2f(self.1, self.0) }
    pub fn abs(self) -> V2f { V2f(self.0.abs(), self.1.abs()) }
    pub fn ang(self) -> Vf { self.1.atan2(self.0) }
    pub fn div_euclid(self, other: V2f) -> V2f { V2f(self.0.div_euclid(other.0), self.1.div_euclid(other.1)) }
    pub fn rem_euclid(self, other: V2f) -> V2f { V2f(self.0.rem_euclid(other.0), self.1.rem_euclid(other.1)) }
    pub fn is_q1(self) -> bool { self.0 >= 0.0 && self.1 >= 0.0 }
    pub fn is_strict_q1(self) -> bool { self.0 > 0.0 && self.1 > 0.0 }
}

impl From<V2i> for V2f {
    fn from(v: V2i) -> V2f { V2f(v.0 as Vf, v.1 as Vf) }
}

impl From<V2f> for V2i {
    fn from(v: V2f) -> V2i { V2i(v.0 as Vi, v.1 as Vi) }
}

macro_rules! impl_binop {
    ($trait:ident, $func:ident, $binop:tt) => {
        impl $trait for V2i {
            type Output = V2i;
            fn $func(self, rhs: V2i) -> V2i { V2i(self.0 $binop rhs.0, self.1 $binop rhs.1) }
        }

        impl $trait for V2f {
            type Output = V2f;
            fn $func(self, rhs: V2f) -> V2f { V2f(self.0 $binop rhs.0, self.1 $binop rhs.1) }
        }
    }
}

impl_binop!(Add, add, +);
impl_binop!(Sub, sub, -);
impl_binop!(Mul, mul, *);
impl_binop!(Div, div, /);
