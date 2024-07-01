use crate::{AcirField, BlackBoxFunctionSolver};

/*
#[cfg(not(feature = "goldilocks"))]
use bn254_blackbox_solver::Bn254BlackBoxSolver;

#[cfg(not(feature = "goldilocks"))]
pub fn default_blackbox_solver<F: AcirField>()
    -> impl BlackBoxFunctionSolver<F> where Bn254BlackBoxSolver: BlackBoxFunctionSolver<F>{
    Bn254BlackBoxSolver
}

#[cfg(feature = "goldilocks")]
*/
pub fn default_blackbox_solver<F: AcirField>() -> impl BlackBoxFunctionSolver<F> {
    use crate::blackbox_solver::StubbedBlackBoxSolver;
    StubbedBlackBoxSolver
}
