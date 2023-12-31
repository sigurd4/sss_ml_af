use super::*;

const SCALE: f64 = 1.7159;
const SENS: f64 = 2.0/3.0;

#[derive(Clone, Copy, Debug)]
pub struct TanhLeCun;

impl<F> BoundingFunction<F> for TanhLeCun
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        (z*f!(SENS)).tanh()*f!(SCALE)
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        let tanh = (z*f!(SENS)).tanh();
        (
            (F::one() - tanh*tanh)*f!(SCALE*SENS),
            []
        )
    }
}

impl_bf!(TanhLeCun);

#[cfg(test)]
mod test
{
    use super::TanhLeCun as BF;
    use crate::tests;

    #[test]
    fn test()
    {
        tests::plot_bf(BF, -5.0..5.0, [])
    }
}