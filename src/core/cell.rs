use crate::core::{RealSpace, GridSpace};
use crate::core::basis::Basis;

use nalgebra::{SMatrix, SVector};

pub const fn num_vertices(real_dims: usize) -> usize { 2usize.pow(real_dims as u32) }

#[derive(Debug)]
pub struct Cell<const R: usize, const I: usize>
where
    [RealSpace<R>; num_vertices(R)]: Sized
{
    pub verts: [RealSpace<R>; num_vertices(R)],
    pub indices: GridSpace<I>,
}

impl<const R: usize, const I: usize> Cell<R, I>
where
    [RealSpace<R>; num_vertices(R)]: Sized,
{
    pub fn from_intersection<B>(intersection: RealSpace<R>,
                                set_nums: &[usize],
                                index_at_intersection: &[isize],
                                basis: &B) -> Self
    where
        B: Basis<R, I>
    {
        let mut indices = basis.gridspace(&intersection);
        // NOTE: Overwrite "known" indices at the intersection this cell is generated from.
        // Avoids an issue with going +/-1 due to the "real" location being exactly on the
        // intersection.

        for (idx, j) in set_nums.iter().enumerate() {
            indices[*j] = index_at_intersection[idx];
        }

        let verts = Self::get_neighbours(&indices, set_nums, index_at_intersection)
            .map(|n| basis.realspace(&n));

        Self { verts, indices }
    }

    /// Returns the grid space indices of the neighbours of an intersection.
    fn epsillon() -> [SVector<isize, R>; num_vertices(R)] {
        let mut eps = [SVector::<isize, R>::zeros(); num_vertices(R)];

        for (i, e) in eps.iter_mut().enumerate() {
            // See write up - each individual bit in the `i` will correspond to if
            // we add 1 in that index.
            for k in 0..R {
                let offset = i >> k & 1;
                e[k] = offset as isize;
            }
        }
        eps
    }

    fn get_neighbours(indices: &GridSpace<I>,
                      j_combos: &[usize],
                      known_indices: &[isize]) -> [GridSpace<I>; num_vertices(R)] {
        let directions = Self::epsillon();
        println!("EPSILLON: {:?}", directions);
        println!("js: {:?}", j_combos);
        
        // Copy initial index to each neighbour.
        let mut neighbours: [GridSpace<I>; num_vertices(R)] = [indices.clone_owned(); num_vertices(R)];

        // Figure out kroneker deltas of which neighbours must be incremented.
        let mut kroneker = SMatrix::<isize, R, I>::zeros();
        for (i, mut kr) in kroneker.row_iter_mut().enumerate() {
            for (j, direction_delta) in kr.iter_mut().enumerate() {
                *direction_delta = (j == j_combos[i]) as isize;
            }
        }

        for (idx, e) in directions.iter().enumerate() {
            let increment = (e.transpose() * &kroneker).transpose();
            println!("DOT {:?} . {:?} = {:?}", e, kroneker, increment);
            neighbours[idx] += increment;
        }
        neighbours
    }

    pub fn numpy_format(&self) -> String {
        let mut out = String::new();
        for vert in self.verts.iter() {
            for idx in 0..vert.len()-1 {
                out += &(vert[idx].to_string() + ", ");
            }
            out += &(vert[vert.len() - 1].to_string() + "\n");
        }

        out
    }
}

