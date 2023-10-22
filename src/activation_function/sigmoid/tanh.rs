use super::*;

#[derive(Clone, Copy)]
pub struct Tanh;

impl<F> BoundingFunction<F> for Tanh
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        z.tanh()
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        let tanh = z.tanh();
        (
            F::one() - tanh*tanh,
            []
        )
    }
}

impl_bf!(Tanh);