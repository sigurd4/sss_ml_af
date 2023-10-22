use std::{f64::consts::FRAC_2_SQRT_PI, f32::consts::FRAC_PI_4};

use super::*;

#[derive(Clone, Copy)]
pub struct Erf;

impl<F> BoundingFunction<F> for Erf
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        f!(libm::erf(z.to_f64().unwrap()/FRAC_2_SQRT_PI))
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            (-z*z*f!(FRAC_PI_4)).exp()*f!(FRAC_2_SQRT_PI),
            []
        )
    }
}

impl_bf!(Erf);