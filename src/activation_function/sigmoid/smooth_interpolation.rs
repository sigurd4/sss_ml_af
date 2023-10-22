use super::*;

#[derive(Clone, Copy)]
pub struct SmoothInterpolation;

impl<F> BoundingFunction<F, 1> for SmoothInterpolation
where
    F: Float
{
    fn bf_y(&self, z: F, [n]: [F; 1]) -> F
    {
        if z.abs() < F::one()
        {
            (n*z/(F::one() - z*z)).tanh()
        }
        else
        {
            z.signum()
        }
    }

    fn bf_dydz(&self, z: F, [n]: [F; 1]) -> (F, [F; 1])
    {
        if z.abs() < F::one()
        {
            let z2 = z*z;
            let one_m_z2 = F::one() - z2;
            let tanh = (n*z/one_m_z2).tanh();
            let sech2 = F::one() - tanh*tanh;

            (
                n*(z2 + F::one())*sech2/(one_m_z2*one_m_z2),
                [z*sech2/one_m_z2]
            )
        }
        else
        {
            (
                F::zero(),
                [F::zero()]
            )
        }
    }
}

impl_bf!(SmoothInterpolation; 1);