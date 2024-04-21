use nalgebra::SVector;

use crate::core::basis::Basis;

const fn num_vertices(real_dims: usize) -> usize { 2usize.pow(real_dims as u32) }

pub struct Cell<const R: usize, const I: usize>
where
    [SVector<f32, R>; num_vertices(R)]: Sized
{
    pub verts: [SVector<f32, R>; num_vertices(R)],
    pub indices: SVector<usize, I>,
}

impl<const R: usize, const I: usize> Cell<R, I>
where
    [SVector<f32, R>; num_vertices(R)]: Sized,
{
    pub fn from_intersection<B>(intersection: SVector<f32, R>,
                                basis: &B) -> Self
    where
        for<'a> &'a B: Basis<R, I>
    {
        let indices = basis.gridspace(&intersection);
        let verts = Self::get_neighbours(&intersection, &basis)
            .map(|n| basis.realspace(&n));

        Self { verts, indices }
    }

    /// Returns the grid space indices of the neighbours of an intersection.
    fn get_neighbours<B>(intersection: &SVector<f32, R>,
                         basis: &B) -> [SVector<usize, I>; num_vertices(R)]
    where
        for<'a> &'a B: Basis<R, I>
    {
        let mut neighbours = [SVector::zeros(); num_vertices(R)];

        for (i, n) in neighbours.iter_mut().enumerate() {
            // See write up - each individual bit in the `i` will correspond to if
            // we add 1 in that index.
            for k in 0..I {
                let offset = i >> k & 1;
                n[k] = offset;
            }
        }
        neighbours
    }
}

