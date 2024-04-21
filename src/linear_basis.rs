use crate::core::{RealSpace, GridSpace};
use crate::core::basis::Basis;
use crate::core::cell::{Cell, num_vertices};

use nalgebra::{SMatrix, SVector, RowVector};

pub struct LinearBasis<const R: usize, const I: usize> {
    /// Each row corresponds to each basis vector's coefficients.
    coefficients: SMatrix<f32, I, R>,
    offsets: SVector<f32, I>,
}

impl<const R: usize, const I: usize> LinearBasis<R, I> {
    pub fn from_vectors(basis_vectors: [RealSpace<R>; I],
                        offsets: [f32; I]) -> Self {
        let coefficients = SMatrix::from_rows(&basis_vectors.map(|v| v.transpose()));
        
        Self { coefficients, offsets: offsets.into() }
    }
}

impl<const R: usize, const I: usize> Basis<R, I> for LinearBasis<R, I> {
    fn gridspace(&self, real_point: &RealSpace<R>) -> GridSpace<I> {
        let mut gsp = (self.coefficients * real_point) - self.offsets;
        for element in gsp.iter_mut() { *element = element.ceil(); }
        nalgebra::convert_ref_unchecked::<SVector<f32, I>, GridSpace<I>>(&gsp)
    }

    fn realspace(&self, grid_space: &GridSpace<I>) -> RealSpace<R> {
        let grid_space_f32 = nalgebra::convert_ref::<GridSpace<I>, SVector<f32, I>>(grid_space);
        self.coefficients.transpose() * grid_space_f32
    }

    fn generate(&self, index_range: usize) -> Vec<Cell<R, I>> where [(); num_vertices(R)]: {
        // Derived
        let total_number_of_intersections = index_range.pow(I as u32);
        let mut cells = Vec::with_capacity(total_number_of_intersections);

        cells
    }
}


