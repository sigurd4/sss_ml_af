use super::*;

#[derive(Clone, Copy, Debug)]
pub struct CLogLog;

impl<F> BoundingFunction<F> for CLogLog
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        F::one() - (-z.exp()).exp()
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        (
            (z - z.exp()).exp(),
            []
        )
    }
}

impl_bf!(CLogLog);

#[cfg(test)]
mod test
{
    use super::CLogLog as BF;
    use crate::tests;

    #[test]
    fn test()
    {
        tests::plot_bf(BF, -5.0..5.0, [])
    }
}