use crate::core::RealSpace;
use crate::linear_basis::LinearBasis;

pub struct BasisFactory {
}

impl BasisFactory {
    pub fn cubic2d(offsets: Option<[f32; 2]>) -> LinearBasis<2, 2> {
        Self::cubic::<2>(offsets)
    }

    pub fn cubic<const D: usize>(offsets: Option<[f32; D]>) -> LinearBasis<D, D> {
        let mut basis_vecs = [RealSpace::<D>::zeros(); D];
        for (j, v) in basis_vecs.iter_mut().enumerate() {
            v[j] = 1.0;
        }

        LinearBasis::<D, D>::from_vectors(basis_vecs, offsets.unwrap_or([0.1; D]))
    }

    pub fn penrose(offsets: Option<[f32; 5]>) -> LinearBasis<2, 5> {
        let mut basis_vecs = [RealSpace::<2>::zeros(); 5];
        let mut angle = 0.0f32;
        let incr = std::f32::consts::PI * 2.0/5.0;

        for j in 0..5 {
            basis_vecs[j] = [angle.cos(), angle.sin()].into();
            angle += incr;
        }
        LinearBasis::<2, 5>::from_vectors(basis_vecs, offsets.unwrap_or([0.1; 5]))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::basis::Basis;
    use crate::core::tools;

    #[test]
    fn cubic() {
        let cubic = BasisFactory::cubic2d(None);
        let cells = cubic.generate(1);
        tools::write_to_file("./cubic.txt", &cells).unwrap();
    }

    #[test]
    fn penrose() {
        let penrose = BasisFactory::penrose(None);

        let cells = penrose.generate(1);
        tools::write_to_file("./penrose.txt", &cells).unwrap();
    }
}

