use super::*;

#[derive(Clone, Copy)]
pub struct ReLUSin;

impl<F> BoundingFunction<F, 2> for ReLUSin
where
    F: Float
{
    fn bf_y(&self, z: F, [omega, phi]: [F; 2]) -> F
    {
        z.max(F::zero()) + if z.is_finite()
        {
            (z*omega + phi).sin()
        }
        else
        {
            F::zero()
        }
    }
    fn bf_dydz(&self, z: F, [omega, phi]: [F; 2]) -> (F, [F; 2])
    {
        let cos = if z.is_finite()
        {
            (z*omega + phi).cos()
        }
        else
        {
            F::zero()
        };

        let uz = if z >= F::zero() {F::one()} else {F::zero()};

        (
            uz + omega*cos,
            [
                cos,
                z*cos
            ]
        )
    }
}

impl_bf!(ReLUSin; 2);