use super::*;

#[derive(Clone, Copy)]
pub struct Softplus;

impl<F> BoundingFunction<F> for Softplus
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        if z > F::zero()
        {
            z + (-z).exp().ln_1p()
        }
        else
        {
            z.exp().ln_1p()
        }
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        let one = F::one();
        (
            if z >= F::zero()
            {
                (one + (-z).exp()).recip()
            }
            else
            {
                one - (one + z.exp()).recip()
            },
            []
        )
    }
}

impl_bf!(Softplus);