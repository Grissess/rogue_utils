use crate::*;

use crate::grid::Grid;

use std::fmt::{self, Debug};
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct Region<T> {
    grid_size: V2i,
    grids: HashMap<V2i, Grid<T>>,
}

pub struct RegionConfig<T> {
    grid_size: V2i,
    _t: PhantomData<T>,
}

#[derive(Debug)]
pub enum Error {
    NonPositiveDim(V2i),
}

impl<T: Clone> Clone for Region<T> {
    fn clone(&self) -> Region<T> {
        Region {
            grid_size: self.grid_size,
            grids: self.grids.clone(),
        }
    }
}

impl<T: Debug> Debug for Region<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Region")
            .field("grid_size", &self.grid_size)
            .field("grids", &self.grids)
            .finish()
    }
}

impl<T> Default for RegionConfig<T> {
    fn default() -> RegionConfig<T> {
        RegionConfig {
            grid_size: V2i(32, 32),
            _t: PhantomData,
        }
    }
}

impl<T> RegionConfig<T> {
    pub fn with_grid_size(self, grid_size: V2i) -> RegionConfig<T> {
        RegionConfig { grid_size, ..self }
    }

    pub fn build(self) -> Result<Region<T>, Error> {
        if !self.grid_size.is_strict_q1() {
            return Err(Error::NonPositiveDim(self.grid_size));
        }
        Ok(Region {
            grid_size: self.grid_size,
            grids: HashMap::new(),
        })
    }
}

impl<T: Default> Region<T> {
    pub fn grid_size(&self) -> V2i {
        self.grid_size
    }

    pub fn grids(&self) -> usize {
        self.grids.len()
    }

    pub fn get_grid_index(&self, v: V2i) -> V2i {
        v.div_euclid(self.grid_size)
    }

    pub fn get_grid_offset(&self, v: V2i) -> V2i {
        v.rem_euclid(self.grid_size)
    }

    pub fn get_grid_mut(&mut self, v: V2i) -> &mut Grid<T> {
        let gi = self.get_grid_index(v);
        let gs = self.grid_size;
        self.grids.entry(gi).or_insert_with(
            || Grid::from_default(
                gi * gs,
                gs
            ).unwrap()
        )
    }

    pub fn get_grid(&self, v: V2i) -> Option<&Grid<T>> {
        self.grids.get(&self.get_grid_index(v))
    }

    pub fn get(&self, v: V2i) -> Option<&T> {
        self.get_grid(v).map(|g| g.get(v).unwrap())
    }

    pub fn get_mut(&mut self, v: V2i) -> &mut T {
        self.get_grid_mut(v).get_mut(v).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initial_allocs() {
        let mut r = RegionConfig::<isize>::default().build().expect("Failed to build Region");
        assert_eq!(r.grids(), 0);
        r.get_mut(V2i(0, 0));
        assert_eq!(r.grids(), 1);
        r.get_mut(r.grid_size() - V2i(1, 1));
        assert_eq!(r.grids(), 1);
    }

    const SIZE: isize = 5;

    #[test]
    fn storage() {
        let mut r = RegionConfig::<isize>::default().build().expect("Failed to build Region");
        assert_eq!(r.grids(), 0);
        for x in -SIZE..SIZE {
            for y in -SIZE..SIZE {
                *r.get_mut(V2i(x, y) * r.grid_size()) = 2 * SIZE * x + y;
            }
        }
        println!("region: {:?}", r);
        assert_eq!(r.grids(), (4 * SIZE * SIZE) as usize);
        for x in -SIZE..SIZE {
            for y in -SIZE..SIZE {
                assert_eq!(r.get(V2i(x, y) * r.grid_size()).unwrap(), &(2 * SIZE * x + y));
            }
        }
    }
}
