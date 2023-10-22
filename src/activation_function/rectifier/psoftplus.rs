use super::*;

#[derive(Clone, Copy)]
pub struct PSoftplus;

impl<F> BoundingFunction<F, 1> for PSoftplus
where
    F: Float
{
    fn bf_y(&self, z: F, [k]: [F; 1]) -> F
    {
        z.max(F::zero()) + if k.is_zero()
        {
            let k_abs = k.abs();
            let z_k = z/k_abs;

            (-z_k.abs()).exp().ln_1p()*k_abs
        }
        else
        {
            F::zero()
        }
    }
    fn bf_dydz(&self, z: F, [k]: [F; 1]) -> (F, [F; 1])
    {
        let k_abs = k.abs();
        let z_k = z/k_abs;

        let exp = (-z_k.abs()).exp();
        let dydz = f!((z >= F::zero()) as u8; F) + (F::one() + exp).recip();
        let y_div_k = exp.ln_1p()*k.signum();

        (
            dydz,
            [{
                if !k.is_zero()
                {
                    y_div_k + z*dydz/k
                }
                else
                {
                    y_div_k
                }
            }]
        )
    }
}

impl_bf!(PSoftplus; 1);