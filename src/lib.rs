#![feature(generic_const_exprs)]

pub mod core;
pub mod linear_basis;
pub mod basis_factory;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::basis_factory::BasisFactory;
    use crate::core::basis::Basis;

    #[test]
    fn space_adjoint_test() {
        // Checks that the gridspace -> realspace -> gridspace is perfect adjoint.
        let basis = BasisFactory::cubic2d(None);
        let gsp = basis.gridspace(&[8.37218362178321, 3.232177412894713289].into());
        let real = basis.realspace(&gsp);

        let answer = basis.gridspace(&real);
        println!("space_adjoint_test: GridSpace {gsp:?} =? Answer {answer:?}");

        assert_eq!(gsp, answer);
    }

    #[test]
    fn cubic() {
        let cubic = BasisFactory::cubic2d(None);
    }
}
