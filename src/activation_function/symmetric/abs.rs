use num::Float;

use super::*;

#[derive(Clone, Copy)]
pub struct Abs;

impl<F> BoundingFunction<F> for Abs
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        z.abs()
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (z.signum(), [])
    }
}

impl_bf!(Abs);