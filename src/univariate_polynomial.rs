use zkstd::common::PrimeField;

#[derive(Debug)]
pub(crate) struct UnivariatePolynomial<F: PrimeField> {
    coeffs: Vec<F>,
}

impl<F: PrimeField> UnivariatePolynomial<F> {
    pub(crate) fn new(coeffs: Vec<F>) -> Self {
        Self { coeffs }
    }

    pub(crate) fn evaluate(&self, at: u64) -> F {
        let value = F::from(at);
        self.coeffs
            .iter()
            .rev()
            .fold(F::zero(), |sum, coeff: &F| sum * value + coeff)
    }
}
