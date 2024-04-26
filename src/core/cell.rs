use crate::core::{RealSpace, GridSpace};
use crate::core::basis::Basis;

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
                                basis: &B) -> Self
    where
        B: Basis<R, I>
    {
        let indices = basis.gridspace(&intersection);
        let zero_real_point = basis.realspace(&indices);

        let verts = Self::get_neighbours()
            .map(|n| zero_real_point + &basis.realspace(&n));

        Self { verts, indices }
    }

    /// Returns the grid space indices of the neighbours of an intersection.
    fn get_neighbours() -> [GridSpace<I>; num_vertices(R)] {
        let mut neighbours = [GridSpace::<I>::zeros(); num_vertices(R)];

        for (i, n) in neighbours.iter_mut().enumerate() {
            // See write up - each individual bit in the `i` will correspond to if
            // we add 1 in that index.
            for k in 0..I {
                let offset = i >> k & 1;
                n[k] = offset as isize;
            }
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

