use std::ops::*;

pub type Vi = isize;
pub type Vf = f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct V2i(pub Vi, pub Vi);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct V2f(pub Vf, pub Vf);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct R2i {
    origin: V2i,
    dim: V2i,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct R2f {
    origin: V2f,
    dim: V2f,
}

impl V2i {
    pub fn l1(self) -> Vi { self.0.abs() + self.1.abs() }
    pub fn l2_sq(self) -> Vi { self.0 * self.0 + self.1 * self.1 }
    pub fn linf(self) -> Vi { self.0.abs().min(self.1.abs()) }
    pub fn swap(self) -> V2i { V2i(self.1, self.0) }
    pub fn abs(self) -> V2i { V2i(self.0.abs(), self.1.abs()) }
    pub fn div_euclid(self, other: V2i) -> V2i { V2i(self.0.div_euclid(other.0), self.1.div_euclid(other.1)) }
    pub fn rem_euclid(self, other: V2i) -> V2i { V2i(self.0.rem_euclid(other.0), self.1.rem_euclid(other.1)) }
    pub fn is_q1(self) -> bool { self.0 >= 0 && self.1 >= 0 }
    pub fn is_strict_q1(self) -> bool { self.0 > 0 && self.1 > 0 }
    pub fn cmin(self) -> Vi { self.0.min(self.1) }
    pub fn cmax(self) -> Vi { self.0.max(self.1) }
    pub fn min(self, other: V2i) -> V2i { V2i(self.0.min(other.0), self.1.min(other.1)) }
    pub fn max(self, other: V2i) -> V2i { V2i(self.0.max(other.0), self.1.max(other.1)) }
}

impl V2f {
    pub fn l1(self) -> Vf { self.0.abs() + self.1.abs() }
    pub fn l2_sq(self) -> Vf { self.0 * self.0 + self.1 * self.1 }
    pub fn l2(self) -> Vf { self.l2_sq().sqrt() }
    pub fn linf(self) -> Vf { self.0.abs().min(self.1.abs()) }
    pub fn swap(self) -> V2f { V2f(self.1, self.0) }
    pub fn abs(self) -> V2f { V2f(self.0.abs(), self.1.abs()) }
    pub fn ang(self) -> Vf { self.1.atan2(self.0) }
    pub fn div_euclid(self, other: V2f) -> V2f { V2f(self.0.div_euclid(other.0), self.1.div_euclid(other.1)) }
    pub fn rem_euclid(self, other: V2f) -> V2f { V2f(self.0.rem_euclid(other.0), self.1.rem_euclid(other.1)) }
    pub fn is_q1(self) -> bool { self.0 >= 0.0 && self.1 >= 0.0 }
    pub fn is_strict_q1(self) -> bool { self.0 > 0.0 && self.1 > 0.0 }
    pub fn cmin(self) -> Vf { self.0.min(self.1) }
    pub fn cmax(self) -> Vf { self.0.max(self.1) }
    pub fn min(self, other: V2f) -> V2f { V2f(self.0.min(other.0), self.1.min(other.1)) }
    pub fn max(self, other: V2f) -> V2f { V2f(self.0.max(other.0), self.1.max(other.1)) }
}

impl From<V2i> for V2f {
    fn from(v: V2i) -> V2f { V2f(v.0 as Vf, v.1 as Vf) }
}

impl From<V2f> for V2i {
    fn from(v: V2f) -> V2i { V2i(v.0 as Vi, v.1 as Vi) }
}

macro_rules! generic_rect {
    ($rect:tt, $vec:tt, $scalar:tt) => {
        impl $rect {
            pub fn origin_dim(mut origin: $vec, mut dim: $vec) -> $rect {
                if dim.0 < 0 as $scalar {
                    dim.0 = -dim.0;
                    origin.0 -= dim.0;
                }
                if dim.1 < 0 as $scalar {
                    dim.1 = -dim.1;
                    origin.1 -= dim.1;
                }
                $rect { origin, dim }
            }

            pub fn origin_opp(origin: $vec, opposite: $vec) -> $rect {
                $rect::origin_dim(origin, opposite - origin)
            }

            pub fn translate(&self, offset: $vec) -> $rect {
                $rect::origin_dim(self.origin + offset, self.dim)
            }
            pub fn scale(&self, amount: $vec) -> $rect {
                $rect::origin_dim(self.origin, self.dim * amount)
            }

            pub fn minor_rad(&self) -> $scalar { self.dim.cmin() }
            pub fn major_rad(&self) -> $scalar { self.dim.cmax() }

            pub fn intersect(&self, other: $rect) -> Option<$rect> {
                let orig = self.origin.max(other.origin);
                let opp = self.opp().min(other.opp());
                let dim = opp - orig;
                if dim.is_strict_q1() {
                    Some($rect::origin_dim(orig, dim))
                } else {
                    None
                }
            }

            pub fn union(&self, other: $rect) -> $rect {
                $rect::origin_opp(self.origin.min(other.origin), self.opp().max(other.opp()))
            }

            pub fn grow_aniso(&self, amt: $vec) -> $rect {
                $rect::origin_opp(self.origin - amt, self.opp() + amt * $vec(2 as $scalar, 2 as $scalar))
            }
            pub fn grow(&self, amt: $scalar) -> $rect {
                self.grow_aniso($vec(amt, amt))
            }

            pub fn origin(&self) -> $vec { self.origin }
            pub fn dim(&self) -> $vec { self.dim }
            pub fn opp(&self) -> $vec { self.origin + self.dim }
        }
    }
}

generic_rect!(R2i, V2i, Vi);
generic_rect!(R2f, V2f, Vf);

#[derive(Debug, Clone, Copy)]
pub struct R2iIter {
    rect: R2i,
    current: V2i,
}

impl Iterator for R2iIter {
    type Item = V2i;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.current;
        let opp = self.rect.opp();

        if cur.1 >= opp.1 {
            return None;
        }

        self.current.0 += 1;
        if self.current.0 >= opp.0 {
            self.current.0 = self.rect.origin().0;
            self.current.1 += 1;
        }

        Some(cur)
    }
}

impl R2i {
    pub fn iter(&self) -> R2iIter {
        R2iIter {
            rect: *self,
            current: self.origin,
        }
    }
}

impl IntoIterator for &R2i {
    type Item = V2i;
    type IntoIter = R2iIter;

    fn into_iter(self) -> R2iIter {
        self.iter()
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rect_iter() {
        let r = R2i::origin_dim(V2i(0, 0), V2i(5, 5));
        let v: Vec<_> = r.iter().collect();
        println!("{:?}", v);
        assert_eq!(v.len(), 25);
    }

    #[test]
    fn rect_isct() {
        let ra = R2i::origin_dim(V2i(0, 0), V2i(5, 5));
        let rb = R2i::origin_dim(V2i(3, 3), V2i(5, 5));

        let isct = ra.intersect(rb).expect("No intersection");
        println!("{:?}", isct);
        assert_eq!(isct.dim(), V2i(2, 2));
    }

    #[test]
    fn rect_isct_disjoint_edge() {
        let ra = R2i::origin_dim(V2i(0, 0), V2i(5, 5));
        let rb = R2i::origin_dim(V2i(5, 5), V2i(5, 5));

        assert!(ra.intersect(rb).is_none());
    }

    #[test]
    fn rect_isct_disjoint() {
        let ra = R2i::origin_dim(V2i(0, 0), V2i(5, 5));
        let rb = R2i::origin_dim(V2i(8, 8), V2i(5, 5));

        assert!(ra.intersect(rb).is_none());
    }

    #[test]
    fn rect_union() {
        let ra = R2i::origin_dim(V2i(0, 0), V2i(5, 5));
        let rb = R2i::origin_dim(V2i(3, 3), V2i(5, 5));

        let un = ra.union(rb);
        println!("{:?}", un);
        assert_eq!(un.dim(), V2i(8, 8));
    }
}
