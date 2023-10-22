use super::*;

#[derive(Clone, Copy)]
pub struct SiLU;

impl<F> BoundingFunction<F> for SiLU
where
    F: Float
{
    fn bf_y(&self, z: F, _: [F; 0]) -> F
    {
        let one = F::one();
        if z >= F::zero()
        {
            z/(one + (-z).exp())
        }
        else
        {
            let exp = z.exp();
            z*exp/(one + exp)
        }
    }
    fn bf_dydz(&self, z: F, _: [F; 0]) -> (F, [F; 0])
    {
        let one = F::one();
        (
            if z > F::zero()
            {
                let exp = (-z).exp();
                let div = exp + one;
                ((z + one)*exp + one)/(div*div)
            }
            else
            {
                let exp = z.exp();
                let div = exp + one;
                exp*(z + exp + one)/(div*div)
            },
            []
        )
    }
}

impl_bf!(SiLU);