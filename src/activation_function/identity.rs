use std::ops::AddAssign;

use num::Float;

use super::*;

#[derive(Clone, Copy)]
pub struct Identity;

impl<F> BoundingFunction<F> for Identity
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        z
    }
    fn bf_dydz(&self, _: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            F::one(),
            []
        )
    }
}

impl<F, const N: usize> ActivationFunction<F, {N - 1}, N> for Identity
where
    F: Float + AddAssign,
    [F; N - 1 + 1]: Into<[F; N]>
{
    fn af_y(&self, x: [F; N - 1], k: [F; N]) -> [F; 1]
    {
        let (w, [b]) = k.rsplit_array();
        [x.mul_dot_bias(w, b)]
    }
    fn af_grad_y(&self, x: [F; N - 1], k: [F; N]) -> [([F; N - 1], [F; N]); 1]
    {
        let w = k.rsplit_array().0;
        [(
            w,
            x.chain([F::one()])
                .into()
        )]
    }
}