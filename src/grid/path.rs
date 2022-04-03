use crate::*;
use super::{Grid, region::{Region, RegionConfig}};

use std::cmp::{Reverse, Ordering};
use std::collections::{BinaryHeap, HashMap};

pub trait Traversable: {
    fn can_pass(&self) -> bool;
}

pub trait Neighbors<T>: Sized {
    fn neighbors(&self, nb: &mut Vec<Self>);
}

#[derive(Debug)]
pub struct L1;
impl Neighbors<L1> for V2i {
    fn neighbors(&self, nb: &mut Vec<V2i>) {
        nb.push(*self + V2i(1, 0));
        nb.push(*self + V2i(0, 1));
        nb.push(*self + V2i(-1, 0));
        nb.push(*self + V2i(0, -1));
    }
}

#[derive(Debug)]
pub struct Linf;
impl Neighbors<Linf> for V2i {
    fn neighbors(&self, nb: &mut Vec<V2i>) {
        nb.push(*self + V2i(1, 0));
        nb.push(*self + V2i(1, 1));
        nb.push(*self + V2i(0, 1));
        nb.push(*self + V2i(-1, 1));
        nb.push(*self + V2i(-1, 0));
        nb.push(*self + V2i(-1, -1));
        nb.push(*self + V2i(0, -1));
        nb.push(*self + V2i(1, -1));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Disconnected,
}

#[derive(Debug)]
struct State {
    node: V2i,
    cost: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool { self.cost == other.cost }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering { self.cost.cmp(&other.cost) }
}

pub fn path<N, A>(start: V2i, goal: V2i, mut allow: A) -> Result<Vec<V2i>, Error>
    where
        V2i: Neighbors<N>,
        A: FnMut(V2i) -> bool
{
    let mut back = HashMap::new();
    let mut cost = HashMap::new();
    let mut open = BinaryHeap::new();
    let mut neighbors = Vec::new();
    
    open.push(Reverse(State { node: start, cost: 0 }));
    cost.insert(start, 0usize);

    while let Some(visit) = open.pop() {
        #[cfg(test)] println!("visit: {:?}", visit);

        let current = visit.0;
        if current.node == goal {
            let mut current = goal;  // NB: shadowed
            let mut path = Vec::new();
            loop {
                #[cfg(test)] println!("current: {:?}", current);

                path.push(current);
                if let Some(next) = back.get(&current) {
                    current = *next;
                } else {
                    path.reverse();
                    return Ok(path);
                }
            }
        }

        current.node.neighbors(&mut neighbors);  // NB: Implicitly using the implementation for N

        for neigh in neighbors.drain(..) {
            if !allow(neigh) {
                continue;
            }


            let est = cost.get(&current.node).unwrap() + 1;  // NB: const 1 cost per traversal assumed
            if !cost.contains_key(&neigh) || est < *cost.get(&neigh).unwrap() {
                cost.insert(neigh, est);
                back.insert(neigh, current.node);
                open.push(Reverse(State { node: neigh, cost: est + (neigh - goal).l1() as usize }));
            }
        }
    }

    Err(Error::Disconnected)
}

impl<T: Traversable> Grid<T> {
    pub fn path<N>(&self, start: V2i, goal: V2i) -> Result<Vec<V2i>, Error>
        where
            V2i: Neighbors<N>
    {
        path::<N, _>(start, goal, |pos| {
            if let Ok(tile) = self.get(pos) {
                tile.can_pass()
            } else {
                false
            }
        })
    }
}

impl<T: Traversable + Default> Region<T> {
    pub fn path<N>(&self, start: V2i, goal: V2i) -> Result<Vec<V2i>, Error>
        where
            V2i: Neighbors<N>
    {
        path::<N, _>(start, goal, |pos| {
            if let Some(tile) = self.get(pos) {
                tile.can_pass()
            } else {
                false
            }
        })
    }

    pub fn path_mut<N>(&mut self, start: V2i, goal: V2i) -> Result<Vec<V2i>, Error>
        where
            V2i: Neighbors<N>
    {
        path::<N, _>(start, goal, |pos| {
            self.get_or_create(pos).can_pass()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Traversable for isize {
        fn can_pass(&self) -> bool { *self == 0 }
    }

    fn testing_grid() -> Grid<isize> {
        Grid::from_vec(
            vec![
                1, 1, 1, 1, 1,
                1, 0, 0, 0, 1,
                1, 0, 1, 0, 1,
                1, 0, 1, 0, 1,
                1, 1, 1, 1, 1,
            ], V2i(0, 0), V2i(5, 5),
        ).expect("Creating the test grid failed")
    }

    #[test]
    fn finds_linf_path() {
        let res = testing_grid().path::<Linf>(V2i(1, 3), V2i(3, 3));
        println!("path: {:?}", res);
        assert!(res.is_ok());
        let path = res.unwrap();
        assert!(path.len() == 5);
        assert_eq!(path.first().unwrap(), &V2i(1, 3));
        assert_eq!(path.last().unwrap(), &V2i(3, 3));
    }
    
    #[test]
    fn finds_l1_path() {
        let res = testing_grid().path::<L1>(V2i(1, 3), V2i(3, 3));
        println!("path: {:?}", res);
        assert!(res.is_ok());
        let path = res.unwrap();
        assert!(path.len() == 7);
        assert_eq!(path.first().unwrap(), &V2i(1, 3));
        assert_eq!(path.last().unwrap(), &V2i(3, 3));
    }

    #[test]
    fn fails_when_disconnected() {
        let mut grid = testing_grid();
        *grid.get_mut(V2i(2, 1)).expect("Failed to get cell") = 1;
        let res = grid.path::<Linf>(V2i(1, 3), V2i(3, 3));
        println!("path: {:?}", res);
        assert!(res.is_err());
    }

    /* Needs to be fixed if ever a closure is passed in again
    #[test]
    fn fails_when_not_allowed() {
        let res = testing_grid().path::<Linf>(V2i(1, 3), V2i(3, 3), |v| (v-V2i(1, 3)).l1() < 3);
        println!("path: {:?}", res);
        assert!(res.is_err());
    }
    */

    #[test]
    fn works_on_regions() {
        let mut reg: Region<isize> = RegionConfig::default().build().unwrap();
        let path = reg.path_mut::<Linf>(V2i(1, 3), V2i(3, 3));
        println!("path: {:?}", path);
        assert!(path.is_ok());
    }
}
