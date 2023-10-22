use num::Float;

use super::*;

#[derive(Clone, Copy)]
pub struct Rem;

impl<F> BoundingFunction<F> for Rem
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        let y = z % F::one();
        if y.is_nan()
        {
            return F::zero()
        }
        y
    }
    fn bf_dydz(&self, _: F, _: [F; 0]) -> (F, [F; 0])
    {
        (F::one(), [])
    }
}

impl_bf!(Rem);