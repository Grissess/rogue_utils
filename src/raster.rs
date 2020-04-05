use crate::*;

#[derive(Debug, Clone)]
pub struct BresenhamLineIter {
    current: V2i,
    delta: V2i,
    error: Vi,
    dist: Vi,
    swap: bool,
}

impl Iterator for BresenhamLineIter {
    type Item = V2i;

    fn next(&mut self) -> Option<Self::Item> {
        if self.dist <= 0 {
            None
        } else {
            let pt = self.current;
            self.dist -= 1;
            self.current.0 += 1;
            if self.error > 0 {
                self.current.1 += self.delta.1.signum();
                self.error -= 2 * self.delta.0;
            }
            self.error += 2 * self.delta.1.abs();

            #[cfg(test)]
            println!("{:?}, {:?}", if self.swap { pt.swap() } else { pt }, self);

            Some(if self.swap { pt.swap() } else { pt })
        }
    }
}

pub fn line(a: V2i, b: V2i) -> BresenhamLineIter {
    let dab = a - b;
    
    let (current, delta, swap) = if dab.1.abs() < dab.0.abs() {
        if a.0 > b.0 {
            (b, a - b, false)
        } else {
            (a, b - a, false)
        }
    } else {
        if a.1 > b.1 {
            (b.swap(), (a - b).swap(), true)
        } else {
            (a.swap(), (b - a).swap(), true)
        }
    };

    BresenhamLineIter {
        current, delta, swap,
        error: 2 * delta.1.abs() - delta.0,
        dist: 1 + if delta.0 == 0 { delta.1 } else { delta.0 },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SIZE: Vi = 5;

    #[test]
    fn axes() {
        for &v in &[V2i(SIZE, 0), V2i(0, SIZE), V2i(-SIZE, 0), V2i(0, -SIZE)] {
            let pts: Vec<_> = line(V2i(0, 0), v).collect();
            println!("zero to {:?}: {:?}", v, pts);
            assert!(pts.contains(&V2i(0, 0)));
            assert!(pts.contains(&v));
        }
    }

    #[test]
    fn diagonals() {
        for &v in &[V2i(SIZE, SIZE), V2i(-SIZE, SIZE), V2i(-SIZE, -SIZE), V2i(SIZE, -SIZE)] {
            let pts: Vec<_> = line(V2i(0, 0), v).collect();
            println!("zero to {:?}: {:?}", v, pts);
            assert!(pts.contains(&V2i(0, 0)));
            assert!(pts.contains(&v));
        }
    }

    #[test]
    fn skews() {
        for &v in &[V2i(SIZE, 2*SIZE), V2i(-3*SIZE, 5*SIZE), V2i(-2*SIZE, -7*SIZE), V2i(2*SIZE, -SIZE)] {
            let pts: Vec<_> = line(V2i(0, 0), v).collect();
            println!("zero to {:?}: {:?}", v, pts);
            assert!(pts.contains(&V2i(0, 0)));
            assert!(pts.contains(&v));
        }
    }

    #[test]
    fn any_endpoint() {
        for &(a, b) in &[
            (V2i(SIZE, 2*SIZE), V2i(-SIZE, -2*SIZE)),
            (V2i(-2*SIZE, -3*SIZE), V2i(0, SIZE)),
            (V2i(0, -SIZE), V2i(0, SIZE)),
            (V2i(3*SIZE, SIZE), V2i(2*SIZE, 2*SIZE)),
        ] {
            let pts: Vec<_> = line(a, b).collect();
            println!("{:?} to {:?}: {:?}", a, b, pts);
            assert!(pts.contains(&a));
            assert!(pts.contains(&b));
        }
    }
}
