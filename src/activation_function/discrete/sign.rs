use num::Float;

use super::*;

#[derive(Clone, Copy)]
pub struct Sign;

impl<F> BoundingFunction<F> for Sign
where
    F: Float
{    
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        z.signum()
    }

    fn bf_dydz(&self, _z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (F::zero(), [])
    }
}

impl_bf!(Sign);