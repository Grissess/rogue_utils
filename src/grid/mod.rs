pub mod path;
pub mod region;

use crate::*;

use std::fmt::{self, Debug};
use std::iter;

pub struct Grid<T> {
    array: Box<[T]>,
    origin: V2i,
    dim: V2i,
}

#[derive(Debug)]
pub enum Error {
    NegativeDim(V2i),
    BadDim(V2i, usize),
    OutOfBounds(V2i),
    BadIndex(usize),
}

impl<T> Grid<T> {
    pub fn from_vec(v: Vec<T>, origin: V2i, dim: V2i) -> Result<Grid<T>, Error> {
        Grid::from_boxed_slice(v.into_boxed_slice(), origin, dim)
    }

    pub fn from_boxed_slice(array: Box<[T]>, origin: V2i, dim: V2i) -> Result<Grid<T>, Error> {
        if !dim.is_q1() {
            return Err(Error::NegativeDim(dim));
        }

        if dim.0 as usize * dim.1 as usize != array.len() {
            return Err(Error::BadDim(dim, array.len()));
        }

        Ok(Grid {
            array, origin, dim,
        })
    }

    pub fn from_generator<G>(mut gen: G, origin: V2i, dim: V2i) -> Result<Grid<T>, Error>
        where
            G: FnMut(V2i) -> T
    {
        if !dim.is_q1() {
            return Err(Error::NegativeDim(dim));
        }

        Grid::from_boxed_slice(
            R2i::origin_dim(origin, dim).iter().map(move |pt| gen(pt)).collect(),
            origin, dim,
        )
    }

    pub fn index_of(&self, v: V2i) -> Result<usize, Error> {
        let d = v - self.origin;
        if d.0 < 0 || d.1 < 0 || d.0 >= self.dim.0 || d.1 >= self.dim.1 {
            return Err(Error::OutOfBounds(v));
        }
        Ok(d.1 as usize * self.dim.0 as usize + d.0 as usize)
    }

    pub fn v2i_of(&self, index: usize) -> Result<V2i, Error> {
        if index >= self.array.len() {
            return Err(Error::BadIndex(index));
        }
        Ok(V2i(index.rem_euclid(self.dim.0 as usize) as isize, index.div_euclid(self.dim.0 as usize) as isize) + self.origin)
    }

    pub fn contains(&self, v: V2i) -> bool { self.index_of(v).is_ok() }

    pub fn get(&self, v: V2i) -> Result<&T, Error> {
        self.index_of(v).map(move |i| &self.array[i])
    }

    pub fn get_mut(&mut self, v: V2i) -> Result<&mut T, Error> {
        self.index_of(v).map(move |i| &mut self.array[i])
    }

    pub fn array(&self) -> &[T] {
        self.array.as_ref()
    }

    pub fn array_mut(&mut self) -> &mut [T] {
        self.array.as_mut()
    }

    pub fn rect(&self) -> R2i {
        R2i::origin_dim(self.origin, self.dim)
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Grid<T> {
        Grid::from_boxed_slice(
            self.array.clone(), self.origin, self.dim,
        ).unwrap()
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Grid")
            .field("array", &self.array)
            .field("origin", &self.origin)
            .field("dim", &self.dim)
            .finish()
    }
}

impl<T: Default> Grid<T> {
    fn from_default(origin: V2i, dim: V2i) -> Result<Grid<T>, Error> {
        if !dim.is_q1() {
            return Err(Error::NegativeDim(dim));
        }

        Grid::from_vec(
            iter::repeat_with(Default::default).take(dim.0 as usize * dim.1 as usize).collect(),
            origin, dim,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SIZE: isize = 5;

    fn testing_grid() -> Grid<isize> {
        Grid::from_default(V2i(0, 0), V2i(SIZE, SIZE)).expect("Creating the test grid failed")
    }

    #[test]
    fn size() {
        assert_eq!(testing_grid().array().len(), SIZE as usize * SIZE as usize);
    }

    #[test]
    fn index_mapping() {
        let grid = testing_grid();
        for i in 0..SIZE*SIZE {
            let pt = grid.v2i_of(i as usize).expect("Failed to get point");
            println!("index {:?} v2i {:?}", i, pt);
            assert_eq!(i as usize, grid.index_of(pt).expect("Failed to get index"));
        }
    }

    #[test]
    fn storage() {
        let mut grid = testing_grid();
        for i in 0..SIZE*SIZE {
            grid.array_mut()[i as usize] = i;
        }
        for i in 0..SIZE*SIZE {
            let pt = grid.v2i_of(i as usize).expect("Failed to get point");
            let val = *grid.get(pt).expect("Failed to index");
            println!("i {:?} v {:?} val {:?}", i, pt, val);
            assert_eq!(i, val);
        }
    }

    #[test]
    fn offset() {
        let grid: Grid<isize> = Grid::from_default(V2i(-3, -3), V2i(SIZE, SIZE)).expect("Creating grid failed");
        for i in 0..SIZE*SIZE {
            let pt = grid.v2i_of(i as usize).expect("Failed to get point");
            println!("offset i {:?} pt {:?}", i, pt);
            assert_eq!(i as usize, grid.index_of(pt).expect("Failed to get index"));
        }
    }
}
