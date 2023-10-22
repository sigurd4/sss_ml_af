use super::*;

#[derive(Clone, Copy)]
pub struct Softminusinv;

impl<F> BoundingFunction<F> for Softminusinv
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        if z < F::zero()
        {
            z - (-z.exp()).ln_1p()
        }
        else
        {
            F::infinity()
        }
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            if z < F::zero()
            {
                (F::one() - z.exp()).recip()
            }
            else
            {
                F::infinity()
            },
            []
        )
    }
}

impl_bf!(Softminusinv);