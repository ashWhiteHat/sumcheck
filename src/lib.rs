mod univariate_polynomial;

use univariate_polynomial::UnivariatePolynomial;
use zkstd::common::PrimeField;

// example polynomial of Justin Thaler's
// https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf#page=37&zoom=100,100,250
struct Polynomial<F: PrimeField> {
    x_1: F,
    x_2: F,
    x_3: F,
}

impl<F: PrimeField> Polynomial<F> {
    fn new(x_1: u64, x_2: u64, x_3: u64) -> Self {
        Self {
            x_1: F::from(x_1),
            x_2: F::from(x_2),
            x_3: F::from(x_3),
        }
    }

    // 2X^3_1 + X_1X_3 + X_2X_3
    fn evaluate(&self) -> F {
        let xxx1 = self.x_1.square() * self.x_1;
        let xxx21 = xxx1.double();
        let x1x3 = self.x_1 * self.x_3;
        let x2x3 = self.x_2 * self.x_3;
        xxx21 + x1x3 + x2x3
    }

    fn s1() -> UnivariatePolynomial<F> {
        let mut coeffs = vec![0; 4];
        // x1, 0, 0
        coeffs[3] += 2;
        // x1, 0, 1
        coeffs[3] += 2;
        coeffs[1] += 1;
        // x1, 1, 0
        coeffs[3] += 2;
        // x1, 1, 1
        coeffs[3] += 2;
        coeffs[1] += 1;
        coeffs[0] += 1;

        UnivariatePolynomial::new(coeffs.iter().map(|coeff| F::from(*coeff)).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jub_jub::Fr as Scalar;

    #[test]
    fn h_test() {
        let a = Polynomial::<Scalar>::new(0, 0, 0).evaluate();
        let b = Polynomial::<Scalar>::new(0, 1, 0).evaluate();
        let c = Polynomial::<Scalar>::new(0, 0, 1).evaluate();
        let d = Polynomial::<Scalar>::new(0, 1, 1).evaluate();
        let e = Polynomial::<Scalar>::new(1, 0, 0).evaluate();
        let f = Polynomial::<Scalar>::new(1, 1, 0).evaluate();
        let g = Polynomial::<Scalar>::new(1, 0, 1).evaluate();
        let h = Polynomial::<Scalar>::new(1, 1, 1).evaluate();

        assert_eq!(Scalar::from(12), a + b + c + d + e + f + g + h)
    }

    #[test]
    fn s1_test() {
        let s1 = Polynomial::<Scalar>::s1();
        println!("{:?}", s1);
        let s10 = s1.evaluate(0);
        let s11 = s1.evaluate(1);
        println!("{:?} {:?}", s10, s11);

        assert_eq!(Scalar::from(12), s10 + s11)
    }
}
