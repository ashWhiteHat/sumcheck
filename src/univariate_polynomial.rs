use zkstd::common::PrimeField;

pub(crate) struct UnivariatePolynomial<F: PrimeField> {
    coeffs: Vec<F>,
}

impl<F: PrimeField> UnivariatePolynomial<F> {
    pub(crate) fn new(coeffs: Vec<F>) -> Self {
        Self { coeffs }
    }
}
