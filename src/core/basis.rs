use nalgebra::SVector;

pub trait Basis<const R: usize, const I: usize> : Sized {
    fn gridspace(&self, real_point: &SVector<f32, R>) -> SVector<usize, I>;
    fn realspace(&self, grid_space: &SVector<usize, I>) -> SVector<f32, R>;
}


