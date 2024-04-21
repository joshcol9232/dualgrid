use crate::core::RealSpace;
use crate::ndbasis::NDBasis;

pub struct BasisFactory {
}

impl BasisFactory {
    pub fn cubic2d(offsets: Option<[f32; 2]>) -> NDBasis<2, 2> {
        Self::cubic::<2>(offsets)
    }

    pub fn cubic<const D: usize>(offsets: Option<[f32; D]>) -> NDBasis<D, D> {
        let mut basis_vecs = [RealSpace::<D>::zeros(); D];
        for (j, v) in basis_vecs.iter_mut().enumerate() {
            v[j] = 1.0;
        }

        NDBasis::<D, D>::new(basis_vecs, offsets.unwrap_or([0.1; D]))
    }
}

