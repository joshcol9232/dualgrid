use crate::core::{RealSpace, GridSpace};

pub trait Basis<const R: usize, const I: usize> : Sized {
    /// Convert a real point into the closest grid space.
    fn gridspace(&self, real_point: &RealSpace<R>) -> GridSpace<I>;
    /// Convert a grid point to it's associated real space.
    fn realspace(&self, grid_space: &GridSpace<I>) -> RealSpace<R>;
}


