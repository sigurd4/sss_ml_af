use std::f64::consts::{FRAC_PI_2, FRAC_2_PI, PI};

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Atan;

impl<F> BoundingFunction<F> for Atan
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        (z*f!(FRAC_PI_2)).atan()*f!(FRAC_2_PI)
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            f!(4.0; F)/(z*z*f!(PI*PI) + f!(4.0)),
            []
        )
    }
}

impl_bf!(Atan);

#[cfg(test)]
mod test
{
    use super::Atan as BF;
    use crate::tests;

    #[test]
    fn test()
    {
        tests::plot_bf(BF, -5.0..5.0, [])
    }
}