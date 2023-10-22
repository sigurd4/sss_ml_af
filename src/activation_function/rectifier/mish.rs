use super::*;

#[derive(Clone, Copy)]
pub struct Mish;

impl<F> BoundingFunction<F> for Mish
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        if z > F::zero()
        {
            let exp = (-z).exp();
            let exp2 = exp*exp;

            let a = exp + F::one();
            let a2 = a*a;

            (a2 - exp2)*z/(a2 + exp2)
        }
        else
        {
            let a = z.exp() + F::one();
            let a2 = a*a;

            (a2 - F::one())*z/(a2 + F::one())
        }
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        let x4 = z*f!(4.0);
        
        (
            if z > F::zero()
            {
                let exp = (-z).exp();
                let exp2 = exp*exp;

                let den = ((exp2 - exp)*f!(2.0) + F::one()).recip();

                (x4*(exp - exp2*f!(2.0))*den + x4 - F::one())*exp2*den + F::one()
            }
            else
            {
                let exp = z.exp();

                let den = f!(2.0; F) - exp*f!(2.0) + exp*exp;

                (x4*(exp - f!(2.0))*den + x4 - F::one())*den + F::one()
            },
            []
        )
    }
}

impl_bf!(Mish);