use super::*;

const SLOPE: f64 = 0.01;

#[derive(Clone, Copy)]
pub struct LReLU;

impl<F> BoundingFunction<F> for LReLU
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        if z >= F::zero()
        {
            z
        }
        else
        {
            z*f!(SLOPE)
        }
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            if z >= F::zero()
            {
                F::one()
            }
            else
            {
                f!(SLOPE)
            },
            []
        )
    }
}

impl_bf!(LReLU);