use acvm::BlackBoxFunctionSolver;


#[cfg(not(feature = "goldilocks"))]
pub fn default_blackbox_solver() -> impl BlackBoxFunctionSolver {
    use bn254_blackbox_solver::Bn254BlackBoxSolver;
    Bn254BlackBoxSolver::new()
}

#[cfg(feature = "goldilocks")]
pub fn default_blackbox_solver() -> impl BlackBoxFunctionSolver {
    use acvm::blackbox_solver::StubbedBlackBoxSolver;
    StubbedBlackBoxSolver
}
