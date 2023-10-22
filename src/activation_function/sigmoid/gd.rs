use std::f64::consts::{FRAC_PI_2, FRAC_2_PI, FRAC_PI_4};

use super::*;

const FRAC_4_PI: f64 = 1.2732395447351626861510701069801;

#[derive(Clone, Copy)]
pub struct Gd;

impl<F> BoundingFunction<F> for Gd
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        (z*f!(FRAC_PI_4)).tanh().atan()*f!(FRAC_4_PI)
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            (z*f!(FRAC_PI_2)).cosh().recip()*f!(FRAC_2_PI),
            []
        )
    }
}

impl_bf!(Gd);