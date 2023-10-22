use super::*;

#[derive(Clone, Copy)]
pub struct Softplusinv;

impl<F> BoundingFunction<F> for Softplusinv
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        if z > F::zero()
        {
            z + (-(-z).exp()).ln_1p()
        }
        else
        {
            F::neg_infinity()
        }
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            if z > F::zero()
            {
                (F::one() - (-z).exp()).recip()
            }
            else
            {
                F::infinity()
            },
            []
        )
    }
}

impl_bf!(Softplusinv);