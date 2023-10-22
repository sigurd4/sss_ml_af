use std::ops::AddAssign;

use num_identities_const::ZeroConst;
use num::Float;

use super::*;

#[derive(Clone, Copy)]
pub struct LogSumExp;

/*impl<F, const N: usize> ActivationFunction<F, N, {N + 1}> for LogSumExp
where
    F: Float + AddAssign,
    [F; N + 1]: ArrayOps<F, {N + 1}, RSplit<1> = [F; N]>
{
    fn af(&self, x: [F; N], k: [F; N + 1]) -> [F; Self::Y]
    {
        let [w, [b]] = k.rsplit_array();
        [
            x.mul_each(w)
                .map2(F::exp)
                .sum_from(b.exp())
                .ln()
        ]
    }

    fn af_grad(&self, x: [F; N], k: [F; N + 1]) -> [([F; N], [F; N + 1]); Self::Y]
    {
        let [w, [b]] = k.rsplit_array();

        let xw_exp = x.mul_each(w)
            .map2(F::exp);
        let b_exp = b.exp();

        let denom_inv = xw_exp.sum_from(b_exp)
            .recip();
        let xw_exp_scaled = xw_exp.mul_all(denom_inv);

        (
            xw_exp_scaled.mul_each(w),
            xw_exp_scaled.mul_each(x),
            b_exp*denom_inv
        )
    }
}*/

impl<F, const N: usize, const K: usize> ActivationFunction<F, N, K> for LogSumExp
where
    F: Float + AddAssign + ZeroConst,
    [(); K/(N + 1)]:,
    [(); 0 - K % (N + 1)]:,
    [(); N + 1]:,
    [(); N + 1 - 1]:,
    [(); N - (N + 1 - 1)]:,
    [(); (N + 1 - 1) - N]:,
    [(); N]:,
    [(); K/(N + 1)*(N + 1)]:,
    [(); K - K/(N + 1)*(N + 1)]:,
    [(); K/(N + 1)*(N + 1) - K]:,
    Identity: ActivationFunction<F, N, {N + 1}, 1>
{
    fn af_y(&self, x: [F; N], k: [F; K]) -> [F; 1]
    {
        let k: [[F; N + 1]; K/(N + 1)] = k.array_chunks_exact();

        [
            k.map2(|k| Identity.af_y(x, k).into_single_item().exp())
                .sum()
                .ln()
        ]
    }

    fn af_grad_y(&self, x: [F; N], k: [F; K]) -> [([F; N], [F; K]); 1]
    {
        let k = k.array_chunks_exact();

        let z_exp = k.map2(|k| Identity.af_y(x, k).into_single_item().exp());

        let denom_inv = z_exp.sum()
            .recip();
        let z_exp_scaled = z_exp.mul_all(denom_inv);

        let w = k.map2(|k| k.rsplit_array::<1>().0.reformulate_length());

        let dydw = z_exp_scaled.mul_outer(&x);
        let dydb = z_exp_scaled;

        let dydk = dydw.comap(dydb, |dydw, dydb| dydw.chain([dydb]))
            .flatten_nd_array();

        [(
            w.transpose()
                .map(|w| z_exp_scaled.mul_dot(w)),
            dydk.reformulate_length()
        )]
    }
}