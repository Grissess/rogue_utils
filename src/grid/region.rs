use crate::*;

use std::fmt::{self, Debug};
use std::collections::HashMap;

pub struct Region<T> {
    grid_size: V2i,
    grids: HashMap<V2i, Grid<T>>,
}

pub struct RegionConfig<T> {
    grid_size: V2i,
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
        }
    }
}

impl<T> RegionConfig<T> {
    pub fn with_grid_size(self, grid_size: V2i) -> RegionConfig<T> {
        RegionConfig { grid_size, .. }
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
    pub fn get_grid_index(&self, v: V2i) -> V2i {
        v.div_euclid(self.grid_size)
    }

    pub fn get_grid_offset(&self, v: V2i) -> V2i {
        v.rem_euclid(self.grid_size)
    }

    pub fn get_grid_mut(&mut self, v: V2i) -> &mut Grid<T> {
        let gi = self.get_grid_index(v);
        self.grids.entry(&gi).or_insert_with(
            || Grid::from_default(
                gi * self.grid_size,
                self.grid_size
            )
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
