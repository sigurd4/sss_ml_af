use super::*;

#[derive(Clone, Copy)]
pub struct Logit;

impl<F> BoundingFunction<F> for Logit
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        if z <= F::zero()
        {
            F::neg_infinity()
        }
        else if z >= F::zero()
        {
            F::infinity()
        }
        else
        {
            (z/(F::one() - z)).ln()
        }
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            if z <= F::zero() || z >= F::one()
            {
                F::infinity()
            }
            else
            {
                (z*(F::one() - z)).recip()
            },
            []
        )
    }
}

impl_bf!(Logit);