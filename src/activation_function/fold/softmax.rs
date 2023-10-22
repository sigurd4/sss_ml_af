use std::ops::AddAssign;

use num_identities_const::ZeroConst;
use num::Float;

use super::*;

#[derive(Clone)]
pub struct Softmax;

impl<F, const N: usize> ActivationFunction<F, N, {N*(N + 1)}, N> for Softmax
where
    F: Float + AddAssign + ZeroConst,
    [[F; N + 1]; N*(N + 1)/(N + 1)]: Into<[[F; N + 1]; N]>,
    [(); 0 - N*(N + 1) % (N + 1)]:,
    [F; N + 1 - 1]: Into<[F; N]>,
    Identity: ActivationFunction<F, N, {N + 1}>
{
    fn af_y(&self, x: [F; N], k: [F; N*(N + 1)]) -> [F; N]
    {
        // Each row corresponds to weights, followed by the bias
        let wb = k.array_chunks_exact()
            .into();

        // The numerator equals e to the power of the biased dot product
        // for each set of weights and biases
        let numer = wb.map2(|wb| Identity.af_y(x, wb)
            .into_single_item()
            .exp()
        );

        // The denominator equals the sum of all the exponentials
        let denom_inv = numer.sum()
            .recip();

        // Now scale all 'numer' by 1/'denom'
        numer.mul_all(denom_inv)
    }

    fn af_grad_y(&self, x: [F; N], k: [F; N*(N + 1)]) -> [([F; N], [F; N*(N + 1)]); N]
    {
        let y = self.af_y(x, k);

        let dydz: [[F; N]; N] = y.enumerate()
            .map_outer(|(i, y_i), (j, y_j)| if i == j
        {
            y_i*(F::one() - y_i)
        }
        else
        {
            -y_i*y_j
        });
        
        // Each row of 'k' corresponds to weights, followed by the bias
        let wb: [[F; N + 1]; N] = k.array_chunks_exact()
            .into();
        let w = wb.map2(|wb| wb.rsplit_array::<1>()
            .0
            .into()
        );
        let w_t = w.transpose();

        dydz.map2(|dydz|
        (
            w_t.map2(|w| dydz.mul_dot(w)),
            dydz.mul_outer(&x.chain([F::one()]))
                .flatten_nd_array()
        ))
    }
}