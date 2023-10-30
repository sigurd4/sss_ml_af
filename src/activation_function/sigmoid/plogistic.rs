use super::*;

#[derive(Clone, Copy, Debug)]
pub struct PLogistic;

impl<F> BoundingFunction<F, 1> for PLogistic
where
    F: Float
{
    fn bf_y(&self, z: F, [alpha]: [F; 1]) -> F
    {
        Logistic.bf_y(z, []).powf(alpha)
    }
    fn bf_dydz(&self, z: F, [alpha]: [F; 1]) -> (F, [F; 1])
    {        
        if z > F::zero()
        {
            let exp_plus_one = F::one() + (-z).exp();
            let frac = exp_plus_one.recip();
            let pow_alpha = frac.powf(alpha);
            (
                alpha*(F::one() - frac)*pow_alpha,
                [-exp_plus_one.ln()*pow_alpha]
            )
        }
        else
        {
            let exp_plus_one = F::one() + z.exp();
            let frac = exp_plus_one.recip();
            let pow_alpha = (F::one() - frac).powf(alpha);
            (
                alpha*frac*pow_alpha,
                [(z - exp_plus_one.ln())*pow_alpha]
            )
        }
    }
}

impl_bf!(PLogistic; 1);

#[cfg(test)]
mod test
{
    use super::PLogistic as BF;
    use crate::tests;

    #[test]
    fn test()
    {
        tests::plot_bf(BF, -5.0..5.0, [1.0])
    }
}