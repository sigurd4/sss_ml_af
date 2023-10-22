use super::*;

#[derive(Clone, Copy)]
pub struct CLogLog;

impl<F> BoundingFunction<F> for CLogLog
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        F::one() - (-z.exp()).exp()
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            (z - z.exp()).exp(),
            []
        )
    }
}

impl_bf!(CLogLog);