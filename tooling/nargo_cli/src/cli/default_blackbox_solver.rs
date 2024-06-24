use acvm::{AcirField, BlackBoxFunctionSolver};


#[cfg(not(feature = "goldilocks"))]
pub fn default_blackbox_solver<F: AcirField>() -> impl BlackBoxFunctionSolver<F> {
    use bn254_blackbox_solver::Bn254BlackBoxSolver;
    Bn254BlackBoxSolver::new()
}

#[cfg(feature = "goldilocks")]
pub fn default_blackbox_solver<F: AcirField>() -> impl BlackBoxFunctionSolver<F> {
    use acvm::blackbox_solver::StubbedBlackBoxSolver;
    StubbedBlackBoxSolver
}
