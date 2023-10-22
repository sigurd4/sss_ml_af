use super::*;

#[derive(Clone, Copy)]
pub struct ELU;

impl<F> BoundingFunction<F, 1> for ELU
where
    F: Float
{
    fn bf_y(&self, z: F, [alpha]: [F; 1]) -> F
    {
        if z < F::zero()
        {
            alpha*(z.exp() - F::one())
        }
        else
        {
            z
        }
    }

    fn bf_dydz(&self, z: F, [alpha]: [F; 1]) -> (F, [F; 1])
    {
        let one = F::one();
        let zero = F::zero();

        if z < zero
        {
            let z_exp = z.exp();
            (
                alpha*z_exp,
                [z_exp - one]
            )
        }
        else
        {
            (
                one,
                [zero]
            )
        }
    }
}

impl_bf!(ELU; 1);