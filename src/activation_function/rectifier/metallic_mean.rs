use super::*;

#[derive(Clone, Copy)]
pub struct MetallicMean;

impl<F> BoundingFunction<F> for MetallicMean
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        (z + (z*z + f!(4.0)).sqrt())*f!(0.5)
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            ((F::one() + z*(z*z + f!(4.0)).sqrt().recip())*f!(0.5)).recip(),
            []
        )
    }
}

impl_bf!(MetallicMean);