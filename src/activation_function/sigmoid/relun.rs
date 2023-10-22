use super::*;

#[derive(Clone, Copy)]
pub struct ReLUN;

impl<F> BoundingFunction<F, 1> for ReLUN
where
    F: Float
{
    fn bf_y(&self, z: F, [n]: [F; 1]) -> F
    {
        let x_sgn_n = z*n.signum();
        let n_abs = n.abs();

        if x_sgn_n > n_abs
        {
            n
        }
        else if x_sgn_n < F::zero()
        {
            F::zero()
        }
        else
        {
            z
        }
    }

    fn bf_dydz(&self, z: F, [n]: [F; 1]) -> (F, [F; 1])
    {
        let x_sgn_n = z*n.signum();
        let n_abs = n.abs();
        
        if x_sgn_n > n_abs
        {
            (
                F::zero(),
                [F::one()]
            )
        }
        else if x_sgn_n < F::zero()
        {
            (
                F::zero(),
                [F::zero()]
            )
        }
        else
        {
            (
                F::one(),
                [F::zero()]
            )
        }
    }
}

impl_bf!(ReLUN; 1);