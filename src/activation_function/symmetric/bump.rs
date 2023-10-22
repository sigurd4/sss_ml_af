use num::Float;

use super::*;

const SQRT_E: f64 = 1.6487212707001281468486507878142;

#[derive(Clone, Copy)]
pub struct Bump;

impl<F> BoundingFunction<F> for Bump
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        let z2 = z*z;
        if z2 < F::one()
        {
            (-(F::zero() - z2).recip()).exp()
        }
        else
        {
            F::zero()
        }
    }

    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        let z2 = z*z;
        
        (
            if z2 < F::one()
            {
                let z2_m1 = z2 - F::one();
                z2_m1.exp2()*F::from(SQRT_E).unwrap()*z/(z2_m1*z2_m1)
            }
            else
            {
                F::zero()
            },
            []
        )
    }
}

impl_bf!(Bump);