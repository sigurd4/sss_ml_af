use super::*;

#[derive(Clone, Copy, Debug)]
pub struct AlgebraicSigmoid;

impl<F> BoundingFunction<F, 1> for AlgebraicSigmoid
where
    F: Float
{
    fn bf_y(&self, z: F, [k]: [F; 1]) -> F
    {
        if k.is_zero()
        {
            F::zero()
        }
        else
        {
            z*(z.abs().powf(k) + F::one()).powf(-k.recip())
        }
    }
    fn bf_dydz(&self, z: F, [k]: [F; 1]) -> (F, [F; 1])
    {
        if k.is_zero()
        {
            (F::zero(), [F::zero()])
        }
        else
        {
            let k_inv = k.recip();

            let z_abs = z.abs();

            let denom = z_abs.powf(k) + F::one();
            let denom_inv = denom.recip();
            let denom_k_inv = denom_inv.powf(k_inv);

            let z_abs_pow_mk_abs = z_abs.powf(-k.abs());
            let z_abs_pow_mk_abs_p1 = z_abs_pow_mk_abs + F::one();
            (
                denom_k_inv*denom_inv,
                [z*(z_abs_pow_mk_abs_p1.ln() - k*z_abs.ln()*z_abs_pow_mk_abs/z_abs_pow_mk_abs_p1)*denom_k_inv/(k*k)]
            )
        }
    }
}

impl_bf!(AlgebraicSigmoid; 1);

#[cfg(test)]
mod test
{
    use super::ReLUN as BF;
    use crate::tests;

    #[test]
    fn test()
    {
        tests::plot_bf(BF, -5.0..5.0, [1.0])
    }
}