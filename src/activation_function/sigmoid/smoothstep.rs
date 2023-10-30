use std::ops::MulAssign;

use polynomial_ops::Polynomial;

use super::*;

use statrs::function::factorial;

#[derive(Clone, Copy, Debug)]
pub struct Smoothstep
{
    pub n: u16
}

impl Smoothstep
{
    pub fn new(n: u16) -> Self
    {
        Self {
            n
        }
    }
}

impl<F> BoundingFunction<F> for Smoothstep
where
    F: Float + MulAssign + Default
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        if z <= F::zero()
        {
            F::zero()
        }
        else if z >= F::one()
        {
            F::one()
        }
        else
        {
            let n = self.n as u64;
            
            let polynomial: Vec<F> = (0..=n).map(|i| f!(factorial::binomial(n + i, i)*factorial::binomial(2*n + 1, n - i)))
                .collect();
            
            polynomial.evaluate_as_polynomial(-z)*z.powi(self.n as i32)
        }
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            if z <= F::zero() || z >= F::one()
            {
                F::zero()
            }
            else
            {
                let n = self.n as u64;
                f!(2*n + 1; F)*f!(factorial::binomial(2*n, n))*((F::one() - z)*z).powi(self.n as i32)
            },
            []
        )
    }
}

impl_bf!(Smoothstep; 0 where F: MulAssign + Default);

#[cfg(test)]
mod test
{
    use super::Smoothstep as BF;
    use crate::tests;

    #[test]
    fn test()
    {
        tests::plot_bf(BF::new(2), -5.0..5.0, [])
    }
}