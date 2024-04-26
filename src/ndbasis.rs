use crate::core::{RealSpace, GridSpace};
use crate::core::basis::Basis;
use crate::core::cell::Cell;

pub struct LinearBasis<const R: usize, const I: usize> {
    basis_vectors: [RealSpace<R>; I],
    offsets: [f32; I],
}

impl<const R: usize, const I: usize> LinearBasis<R, I> {
    pub fn new(basis_vectors: [RealSpace<R>; I],
               offsets: [f32; I]) -> Self {
        Self { basis_vectors, offsets }
    }
}

impl<const R: usize, const I: usize> Basis<R, I> for LinearBasis<R, I> {
    fn gridspace(&self, real_point: &RealSpace<R>) -> GridSpace<I> {
        let mut gsp = GridSpace::<I>::zeros();
        for (j, e) in self.basis_vectors.iter().enumerate() {
            gsp[j] = ( real_point.dot(&self.basis_vectors[j]) - self.offsets[j] ) as isize;
        }
        gsp
    }

    fn realspace(&self, grid_space: &GridSpace<I>) -> RealSpace<R> {
        let mut real = RealSpace::<R>::zeros();
        for (j, e) in self.basis_vectors.iter().enumerate() {
            real += e * grid_space[j] as f32;
        }
        real
    }

    fn generate(&self, index_range: usize) -> Vec<Cell<R, I>> {
        // Derived
        let total_number_of_intersections = index_range.pow(self.basis_vectors.len());
        let mut out = Vec::with_capacity(total_number_of_intersections);


        for e in self.basis_vectors.iter() {
        }

    }
}


