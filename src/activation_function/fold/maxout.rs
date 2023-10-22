use std::ops::AddAssign;

use num::Float;

use super::*;

pub struct Maxout<const K: usize>;

impl<F, const N: usize, const K: usize> ActivationFunction<F, N, K> for Maxout<K>
where
    F: Float + AddAssign,
    [(); K/(N + 1)]:,
    [(); 0 - K % (N + 1)]:,
    [(); N + 1 - 1]:,
    [(); (N + 1 - 1) - N]:,
    [(); N - (N + 1 - 1)]:,
    [(); K/(N + 1)*(N + 1)]:,
    [(); K - K/(N + 1)*(N + 1)]:,
    [(); K/(N + 1)*(N + 1) - K]:,
    Identity: ActivationFunction<F, N, {N + 1}>
{
    fn af_y(&self, x: [F; N], k: [F; K]) -> [F; 1]
    {
        // Each row corresponds to weights, followed by the bias
        let wb = k.array_chunks_exact();

        // Compute the maximum of the biased dot product for each set of weights and biases
        [
            wb.map2(|wb| Identity.af_y(x, wb).into_single_item())
                .reduce(F::max)
                .unwrap_or(F::zero())
        ]
    }
    fn af_grad_y(&self, x: [F; N], k: [F; K]) -> [([F; N], [F; K]); 1]
    {
        // Each row corresponds to weights, followed by the bias
        let wb = k.array_chunks_exact();

        let mut dydk = [[F::zero(); N + 1]; K/(N + 1)];

        // m = argmax of I(x, w, b)
        let dydx = if let Some(m) = wb.map2(|k| Identity.af_y(x, k))
            .argmax()
        {
            dydk[m] = x.chain([F::one()]); // dydw = x, dydb = 1.0
            
            (&wb[m]).rsplit_array::<1>().0
                .reformulate_length() // w[m]
        }
        else
        {
            [F::zero(); N]
        };
        
        [(dydx, dydk.flatten_nd_array().reformulate_length())]
    }
}