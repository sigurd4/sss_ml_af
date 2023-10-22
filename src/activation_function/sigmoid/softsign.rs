use super::*;

#[derive(Clone, Copy)]
pub struct Softsign;

impl<F> BoundingFunction<F> for Softsign
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        z/(F::one() + z.abs())
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        let sqrt = (F::one() + z.abs()).recip();
        (
            sqrt*sqrt,
            []
        )
    }
}

impl_bf!(Softsign);