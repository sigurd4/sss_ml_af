use super::*;

#[derive(Clone, Copy)]
pub struct Softminus;

impl<F> BoundingFunction<F> for Softminus
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        let one = F::one();
        if z >= F::zero()
        {
            -(one + (-z).exp()).ln()
        }
        else
        {
            z - (one + z.exp()).ln()
        }
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        let one = F::one();
        (
            if z <= F::zero()
            {
                (one + z.exp()).recip()
            }
            else
            {
                one - (one + (-z).exp()).recip()
            },
            []
        )
    }
}

impl_bf!(Softminus);