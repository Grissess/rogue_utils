use crate::*;

use crate::grid::Grid;

use std::fmt::{self, Debug};
use std::collections::HashMap;
use std::marker::PhantomData;

/* Arguments: Invoking point, Region coordinate, Grid origin, Grid dim */
type GridGen<T> = Box<dyn FnMut(V2i, V2i, V2i, V2i) -> Grid<T>>;

pub struct Region<T> {
    grid_size: V2i,
    grids: HashMap<V2i, Grid<T>>,
    grid_gen: Option<GridGen<T>>,
}

pub struct RegionConfig<T> {
    grid_size: V2i,
    grid_gen: Option<GridGen<T>>,
    _t: PhantomData<T>,
}

#[derive(Debug)]
pub enum Error {
    NonPositiveDim(V2i),
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
            grid_gen: None,
            _t: PhantomData,
        }
    }
}

impl<T> RegionConfig<T> {
    pub fn with_grid_size(self, grid_size: V2i) -> RegionConfig<T> {
        RegionConfig { grid_size, ..self }
    }

    pub fn with_grid_gen(self, grid_gen: Option<GridGen<T>>) -> RegionConfig<T> {
        RegionConfig { grid_gen, ..self }
    }

    pub fn build(self) -> Result<Region<T>, Error> {
        if !self.grid_size.is_strict_q1() {
            return Err(Error::NonPositiveDim(self.grid_size));
        }
        Ok(Region {
            grid_size: self.grid_size,
            grids: HashMap::new(),
            grid_gen: self.grid_gen,
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
        let gg = self.grid_gen.as_mut();
        self.grids.entry(gi).or_insert_with(||
            match gg {
                Some(gen) => gen(v, gi, gi * gs, gs),
                None => Grid::from_default(
                    gi * gs,
                    gs
                ).unwrap(),
            }
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

    pub fn get_or_create(&mut self, v: V2i) -> &T {
        self.get_mut(v)  // NB: Downgrades
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

    #[test]
    fn generator_grid() {
        let mut r = RegionConfig::<isize>::default().with_grid_gen(Some(Box::new(|_, _, o, d|
            Grid::from_generator(|g: V2i| g.l1(), o, d).expect("Failed to generate Grid")
        ))).build().expect("Failed to build Region");
        
        for pt in &R2i::origin_dim(V2i(0, 0), r.grid_size()) {
            assert_eq!(*r.get_or_create(pt), pt.l1());
        }

        println!("{:?}", r);
    }

    #[test]
    fn generator_region() {
        let mut r = RegionConfig::<isize>::default().with_grid_gen(Some(Box::new(|_, r, o, d|
            Grid::from_generator(|_| r.l1(), o, d).expect("Failed to generate Grid")
        ))).build().expect("Failed to build Region");
        
        for pt in &R2i::origin_dim(V2i(0, 0), r.grid_size()) {
            assert_eq!(*r.get_or_create(pt), 0);
        }
        for pt in &R2i::origin_dim(r.grid_size(), r.grid_size()) {
            assert_eq!(*r.get_or_create(pt), 2);
        }

        println!("{:?}", r);
    }

    #[test]
    fn generator_invoker() {
        let mut r = RegionConfig::<isize>::default().with_grid_gen(Some(Box::new(|i, _, o, d|
            Grid::from_generator(|_| i.l1(), o, d).expect("Failed to generate Grid")
        ))).build().expect("Failed to build Region");

        let ipt = V2i(5, 5);

        r.get_or_create(ipt);
        
        for pt in &R2i::origin_dim(V2i(0, 0), r.grid_size()) {
            assert_eq!(*r.get_or_create(pt), ipt.l1());
        }

        println!("{:?}", r);
    }
}
