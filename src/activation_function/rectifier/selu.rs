use super::*;

#[derive(Clone, Copy)]
pub struct SELU;

/*const ALPHA: f64 = 1.67326;
const LAMBDA: f64 = 1.0507;

impl<F> BoundingFunction<F> for SELU
where
    F: Float
{
    fn y(&self, z: F, _: [F; 0]) -> F
    {
        f!(LAMBDA; F)*if z < F::zero()
        {
            (z.exp() - F::one())*f!(ALPHA)
        }
        else
        {
            z
        }
    }
    fn dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        if z < F::zero()
        {
            (
                (z.exp() - F::one())*f!(LAMBDA*ALPHA),
                []
            )
        }
        else
        {
            (
                f!(LAMBDA),
                []
            )
        }
    }
}*/

impl<F> BoundingFunction<F, 2> for SELU
where
    F: Float
{
    fn bf_y(&self, z: F, [alpha, lambda]: [F; 2]) -> F
    {
        lambda*if z < F::zero()
        {
            alpha*(z.exp() - F::one())
        }
        else
        {
            z
        }
    }
    fn bf_dydz(&self, z: F, [alpha, lambda]: [F; 2]) -> (F, [F; 2])
    {
        if z < F::zero()
        {
            let exp_m1 = z.exp() - F::one();
            (
                lambda*alpha*z.exp(),
                [
                    lambda*exp_m1,
                    alpha*exp_m1
                ]
            )
        }
        else
        {
            (
                lambda,
                [
                    F::zero(),
                    z
                ]
            )
        }
    }
}

impl_bf!(SELU; 2);