use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Logistic;

impl<F> BoundingFunction<F> for Logistic
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        // Alternative 1:
        //(F::one() + (-z).exp()).recip()

        // Alternative 2:
        if z >= F::zero()
        {
            (F::one() + (-z).exp()).recip()
        }
        else
        {
            F::one() - (F::one() + z.exp()).recip()
        }
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        let exp = (-z.abs()).exp();
        let div_inv = (F::one() + exp).recip();
        (
            exp*div_inv*div_inv,
            []
        )
    }
}

impl_bf!(Logistic);

#[cfg(test)]
mod test
{
    use super::Logistic as BF;
    use crate::tests;

    #[test]
    fn test()
    {
        tests::plot_bf(BF, -5.0..5.0, [])
    }
}