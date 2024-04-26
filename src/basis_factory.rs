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
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::basis::Basis;

    #[test]
    fn cubic() {
        let cubic = BasisFactory::cubic2d(None);
        //let cells = cubic.generate(1);
    }

}

