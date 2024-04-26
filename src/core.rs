pub mod basis;
pub mod cell;
pub mod tools;

use nalgebra::SVector;

/// A location within the gridspace of the multigrid. Each index corresponds
/// to a multiple of the corresponding basis vector.
pub type GridSpace<const I: usize> = SVector<isize, I>;
/// Locations in real space within the grid.
pub type RealSpace<const R: usize> = SVector<f32, R>;

