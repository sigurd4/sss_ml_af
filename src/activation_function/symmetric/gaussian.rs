use num::Float;

use super::*;

#[derive(Clone, Copy)]
pub struct Gaussian;

impl<F> BoundingFunction<F> for Gaussian
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        (-z*z).exp()
    }

    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            -f!(2.0; F)*z*(-z*z).exp(),
            []
        )
    }
}

impl_bf!(Gaussian);