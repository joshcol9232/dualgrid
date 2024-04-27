use crate::core::RealSpace;
use crate::linear_basis::LinearBasis;

const fn fix_even(n: usize) -> usize {
    if n % 2 == 0 { n / 2 } else { n }
}

pub struct BasisFactory {
}

impl BasisFactory {
    fn calc_surf_vectors_rotsym<const S: usize>() -> [RealSpace<2>; fix_even(S)] {
        let mut basis_vecs = [RealSpace::<2>::zeros(); fix_even(S)];
        let mut angle = 0.0f32;
        let incr = std::f32::consts::PI * 2.0/5.0;

        for j in 0..fix_even(S) {
            basis_vecs[j] = [angle.cos(), angle.sin()].into();
            angle += incr;
        }
        basis_vecs
    }

    fn penrose_constraint<const I: usize>(mut offsets: [f32; I]) -> [f32; I] {
        // Penrose constraint is that the sum of the offsets is 0.
        // In order to sum to 0, make last offset equal to negative sum
        // of the other numbers.
        let mut sum_bar_last = 0.0;
        for idx in 0..offsets.len() - 1 {
            sum_bar_last += offsets[idx];
        }
        offsets[offsets.len() - 1] = -sum_bar_last;
        offsets
    }

    pub fn surf_with_rotsym<const S: usize>(offsets: Option<[f32; fix_even(S)]>) -> LinearBasis<2, { fix_even(S) }> {
        LinearBasis::<2, {fix_even(S)}>::from_vectors(Self::calc_surf_vectors_rotsym::<S>(), offsets.unwrap_or([0.1; fix_even(S)]))
    }

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
        // Penrose is a special case where offsets sum to 0.
        let offsets_clamped = if let Some(ofst) = offsets {
            Self::penrose_constraint(ofst)
        } else {
            [0.2, 0.2, 0.2, 0.2, -0.8]
        };
        LinearBasis::<2, 5>::from_vectors(Self::calc_surf_vectors_rotsym::<5>(),
                                          offsets_clamped)
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

        let cells = penrose.generate(10);
        tools::write_to_file("./penrose.txt", &cells).unwrap();
    }
}

