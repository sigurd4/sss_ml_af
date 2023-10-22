use super::*;

#[derive(Clone, Copy)]
pub struct Clip;

impl<F> BoundingFunction<F> for Clip
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        z.min(F::one()).max(-F::one())
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            f!((z < F::one() && z > -F::one()) as u8),
            []
        )
    }
}

impl_bf!(Clip);