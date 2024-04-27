#![feature(generic_const_exprs)]

pub mod core;
pub mod linear_basis;
pub mod basis_factory;
pub mod filtering;

#[cfg(test)]
mod tests {
    use super::*;
    /*
    #[test]
    fn cubic() {
        let cubic = BasisFactory::cubic2d(None);
    }
    */
}
