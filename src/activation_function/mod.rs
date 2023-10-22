use num::Float;

moddef::moddef!(
    flat(pub) mod {
        identity
    },
    pub mod {
        discrete,
        periodic,
        symmetric,
        sigmoid,
        rectifier,

        fold
    }
);

use super::*;

pub trait ActivationFunctionObj<F>
{
    fn af_dyn(&self, x: &[F], k: &[F]) -> Vec<F>;
    fn af_grad_dyn(&self, x: &[F], k: &[F]) -> Vec<(F, Vec<F>)>;
}

pub trait ActivationFunction<F, const X: usize, const K: usize, const Y: usize = {1}>
{
    fn af_y(&self, x: [F; X], k: [F; K]) -> [F; Y];
    fn af_grad_y(&self, x: [F; X], k: [F; K]) -> [([F; X], [F; K]); Y];
}

pub trait BoundingFunction<F, const K: usize = 0>
{
    fn bf_y(&self, z: F, k: [F; K]) -> F;
    fn bf_dydz(&self, z: F, k: [F; K]) -> (F, [F; K]);
}

#[macro_export]
macro_rules! impl_bf {
    ($bf:ident; $k:literal $(where $($where:tt)+)?) => {
        impl<F, const N: usize, const K: usize> ActivationFunction<F, N, K> for $bf
        where
            F: Float,
            Identity: ActivationFunction<F, N, {K - $k}>,
            [F; K - $k + $k]: Into<[F; K]>,
            $($($where)+)?
        {
            fn af_y(&self, x: [F; N], k: [F; K]) -> [F; 1]
            {
                let (wb, k) = k.rsplit_array();
                [self.bf_y(Identity.af_y(x, wb).into_single_item(), k)]
            }
            fn af_grad_y(&self, x: [F; N], k: [F; K]) -> [([F; N], [F; K]); 1]
            {
                let (wb, k) = k.rsplit_array();
                let (dydz, dydk) = self.bf_dydz(Identity.af_y(x, wb).into_single_item(), k);
                let [(dzdx, dzdwb)] = Identity.af_grad_y(x, wb);
                [(
                    dzdx.mul_all(dydz),
                    dzdwb.mul_all(dydz)
                        .chain(dydk)
                        .into()
                )]
            }
        }
    };
    ($bf:ident; $k:literal, $($k_next:literal),+ $(where $($where:tt)+)?) => {
        impl_bf!($bf; $k $(where $($where)+)?);

        impl_bf!($bf; $($k_next),+ $(where $($where)+)?);
    };
    ($bf:ident $(where $($where:tt)+)?) => {
        impl_bf!($bf; 0 $(where $($where)+)?);
    };
}

/*impl<T, F, const N: usize> ActivationFunction<[F; N], [F; N], F> for T
where
    T: BoundingFunction<F, Output = F>,
    F: Float + AddAssign
{
    type Output = F;
    type OutputGradX = [F; N];
    type OutputGradW = [F; N];
    type OutputGradB = F;

    fn f(&self, x: [F; N], w: [F; N], b: F) -> Self::Output
    {
        let z = x.mul_dot_bias(w, b);
        let y = self.y(z);
        if y.is_nan()
        {
            return F::zero()
        }
        y
    }

    fn f_grad(&self, x: [F; N], w: [F; N], b: F) -> (Self::OutputGradX, Self::OutputGradW, Self::OutputGradB)
    {
        let z = Identity.f(x, w, b);
        let dydz = self.dydz(z);
        if dydz.is_nan()
        {
            return ([F::zero(); N], [F::zero(); N], F::zero())
        }
        let (dzdx, dzdw, dzdb) = (w, x, F::one());
        (
            dzdx.mul_all(dydz),
            dzdw.mul_all(dydz),
            dzdb*dydz
        )
    }
}*/