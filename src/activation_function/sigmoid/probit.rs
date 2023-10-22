use std::f64::consts::SQRT_2;

use statrs::consts::SQRT_2PI;

use super::*;

#[derive(Clone, Copy)]
pub struct Probit;

impl<F> BoundingFunction<F> for Probit
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        if z <= F::zero()
        {
            F::neg_infinity()
        }
        else if z >= F::one()
        {
            F::infinity()
        }
        else
        {
            f!(SQRT_2*statrs::function::erf::erf_inv(2.0*z.to_f64().unwrap() - 1.0))
        }
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            if z > F::zero() && z < F::one()
            {
                let z2_m1 = 2.0*z.to_f64().unwrap() - 1.0;
                f!(SQRT_2PI*statrs::function::erf::erf_inv(z2_m1*z2_m1).exp())
            }
            else
            {
                F::infinity()
            },
            []
        )
    }
}

impl_bf!(Probit);