use num::Float;

use super::*;

#[derive(Clone, Copy)]
pub struct Sin;

impl<F> BoundingFunction<F> for Sin
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        let y = z.sin();
        if y.is_nan()
        {
            return F::zero()
        }
        y
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        let y = z.cos();
        if y.is_nan()
        {
            return (F::zero(), [])
        }
        (y, [])
    }
}

impl_bf!(Sin);