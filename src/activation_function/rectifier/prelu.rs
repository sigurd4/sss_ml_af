use super::*;

#[derive(Clone, Copy)]
pub struct PReLU
{
    pub a: f64
}

impl<F> BoundingFunction<F, 1> for PReLU
where
    F: Float
{
    fn bf_y(&self, z: F, [a]: [F; 1]) -> F
    {
        if z >= F::zero()
        {
            z
        }
        else
        {
            z*a
        }
    }
    fn bf_dydz(&self, z: F, [a]: [F; 1]) -> (F, [F; 1])
    {
        let zero = F::zero();

        if z >= zero
        {
            (
                F::one(),
                [zero]
            )
        }
        else
        {
            (
                a,
                [z]
            )
        }
    }
}

impl_bf!(PReLU; 1);