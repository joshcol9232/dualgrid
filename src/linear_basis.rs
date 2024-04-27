use crate::core::{RealSpace, GridSpace};
use crate::core::basis::Basis;
use crate::core::cell::{Cell, num_vertices};

use nalgebra::{SMatrix, SVector, DMatrixView, DVectorView};
use itertools::Itertools;

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

fn get_index_ranges<const R: usize, const I: usize>(max_range: isize) -> impl Iterator<Item = SVector<f32, R>> {
    // For each set of R dimensions, go through [-k, k] in construction set.
    // E.g k = 1, in 2d:
    // ((-1, 0, 1) (-1, 0, 1))
    // NOTE: As f32, as this gets added to other vectors of f32.
    (0..R).map(|_| (-max_range..=max_range).map(|int| int as f32))
    // Then, cartesian product of these two sets gives all combinations between sets.
          .multi_cartesian_product()
          .map(|range_vec| SVector::<f32, R>::from_vec(range_vec))

    // -> { [-1, -1], [-1,  0], [-1,  1],
    //      [ 0, -1], [ 0,  0], [ 0,  1],
    //      [ 1, -1], [ 1,  0], [ 1,  1] }
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
        let index_combos = get_index_ranges::<R, I>(index_range as isize)
            .collect::<Vec<SVector<f32, R>>>();

        let mut cells = Vec::with_capacity(total_number_of_intersections);

        // For each combination of "construction sets", there are a set of intersections.
        // Each combination must be between R different sets (the number of dimensions).
        // 2D -> Two non-parallel lines must meet at a point.
        // 3D -> Three non-parallel planes must meet at a point.
        // 4D -> ...
        // `j` always denotes the set number / basis vector number.

        for j_combination in (0..I).combinations(R) {
            // Make a view, removing the rows we aren't looking at.
            let ignore_indices = (0..I).filter(|n| !j_combination.iter().any(|j| j == n))
                                       .collect::<Vec<usize>>();
            let intersection_matrix = {
                let coef_view: DMatrixView<f32> = self.coefficients.as_view();
                coef_view.remove_rows_at(&ignore_indices)
            };

            let offsets_mask = {
                let offsets_view: DVectorView<f32> = self.offsets.as_view();
                offsets_view.remove_rows_at(&ignore_indices)
            };

            if let Some(inverse_intersection_mat) = intersection_matrix.try_inverse() {
                // Iterate through K ranges!
                for k_range in index_combos.iter() {
                    let intersection: RealSpace<R> = (&inverse_intersection_mat * (&offsets_mask + k_range))
                        .fixed_view::<R, 1>(0, 0)  // This view should be the same shape as the
                                                   // maths...
                        .into_owned();

                    let k_range_indices = nalgebra::convert_ref_unchecked::<SVector<f32, R>, SVector<isize, R>>(&k_range);
                    let cell = Cell::from_intersection(intersection, &j_combination, k_range_indices.as_slice(), self);
                    cells.push(cell);
                }
            } else {
                println!("Singular matrix!");
                // Skip this one - N-d planes do not intersect at a point.
            }
        }

        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cubic2d() -> LinearBasis<2, 2> {
        LinearBasis::from_vectors([[1.0, 0.0].into(), [0.0, 1.0].into()],
                                  [0.1, 0.1])
    }

    #[test]
    fn grid_to_real() {
        let basis = cubic2d();
        let gsp: GridSpace<2> = [1, 2].into();
        let real = basis.realspace(&gsp);
        // Real in cubic should be the same as grid space, but rounded.
        for n in 0..2 {
            let x_g: f32 = gsp[n] as f32;
            let x_r: f32 = real[n];
            assert!(x_g == x_r,
                    "linear_basis::grid_to_real: RealSpace[{}] is not equal to GridSpace[{}] 
                     for cubic case. (G {}, R {})", n, n, x_g, x_r);
        }
    }

    #[test]
    fn real_to_grid() {
        let basis = cubic2d();
        let real: RealSpace<2> = [1.2, 2.3].into();
        let gsp = basis.gridspace(&real);
        for n in 0..2 {
            let real_ceil = real[n].ceil() as isize;
            assert!(gsp[n] == real_ceil,
                    "linear_basis::real_to_grid: GridSpace[{}] is not equal to RealSpace[{}]
                     for cubic case. (R {}, G {})", n, n, real_ceil, gsp[n]);
        }
    }

    #[test]
    fn space_adjoint_test() {
        let basis = cubic2d();
        // Checks that the gridspace -> realspace -> gridspace is perfect adjoint.
        let gsp = basis.gridspace(&[8.37218362178321, 3.232177412894713289].into());
        let real = basis.realspace(&gsp);

        let answer = basis.gridspace(&real);
        println!("space_adjoint_test: GridSpace {gsp:?} =? Answer {answer:?}");

        assert_eq!(gsp, answer);
    }

    #[test]
    fn linear_simple() {
        use crate::core::tools;

        println!("LINEAR");
        let lin = LinearBasis::from_vectors([[1.0, 0.0].into(),
                                             [0.0, 1.0].into(),
                                             [1.0/2.0f32.sqrt(), 1.0/2.0f32.sqrt()].into()],
                                             [0.1, 0.1, 0.1]);
        let cells = lin.generate(1);
        println!("CELLS: {:?}", cells);
        tools::write_to_file("linear_out.txt", &cells).unwrap();

        println!("LINEAR");
    }
}


