use super::*;

#[derive(Clone, Copy)]
pub struct Swish;

impl<F> BoundingFunction<F, 1> for Swish
where
    F: Float
{
    fn bf_y(&self, z: F, [beta]: [F; 1]) -> F
    {
        let one = F::one();
        let bz = beta*z;
        if bz >= F::zero()
        {
            z/(one + (-bz).exp())
        }
        else
        {
            let exp = bz.exp();
            z*exp/(one + exp)
        }
    }
    fn bf_dydz(&self, z: F, [beta]: [F; 1]) -> (F, [F; 1])
    {
        let one = F::one();
        let bz = beta*z;
        if bz > F::zero()
        {
            let exp = (-bz).exp();
            let div = exp + one;
            let denom = (div*div).recip();

            (
                (bz*exp + div)*denom,
                [z*z*exp*denom]
            )
        }
        else
        {
            let exp = bz.exp();
            let div = exp + one;
            let denom = (div*div).recip();

            (
                exp*(bz + div)*denom,
                [z*z*exp*denom]
            )
        }
    }
}

impl_bf!(Swish; 1);