use std::f64::consts::FRAC_1_SQRT_2;

use super::*;

const APPROXIMATION: bool = true;

const A: f64 = 0.04495641;
const FRAC_SQRT_2_SQRT_PI: f64 = 0.7978845608028653558798921198687637369517172623298693153318516593;
const FRAC_1_SQRT_2PI: f64 = 0.3989422804014326779399460599343818684758586311649346576659258296;

#[derive(Clone, Copy)]
pub struct GELU;

impl GELU
{
    fn erfc_a<F>(z: F) -> F
    where
        F: Float
    {
        f!(0.5; F)*if !APPROXIMATION
        {
            f!(libm::erfc(-z.to_f64().unwrap()*FRAC_1_SQRT_2))
        }
        else
        {
            F::one() + (z*f!(FRAC_SQRT_2_SQRT_PI)*(F::one() + z*z*f!(A))).tanh()
        }
    }
}

impl<F> BoundingFunction<F> for GELU
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        GELU::erfc_a(z)*z
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            GELU::erfc_a(z) + z*f!(FRAC_1_SQRT_2PI)*(-z*z*f!(0.5)).exp(),
            []
        )
    }
}

impl_bf!(GELU);