use super::*;

#[derive(Clone, Copy)]
pub struct ReLU;

impl<F> BoundingFunction<F> for ReLU
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        z.max(F::zero())
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            if z >= F::zero() {F::one()} else {F::zero()},
            []
        )
    }
}

impl_bf!(ReLU);