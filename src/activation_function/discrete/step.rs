use num::Float;

use super::*;

#[derive(Clone, Copy)]
pub struct Step;

impl<F> BoundingFunction<F> for Step
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        f!(z.is_sign_positive() as u8)
    }

    fn bf_dydz(&self, _z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (F::zero(), [])
    }
}

impl_bf!(Step);