use std::f64::INFINITY;

use num::Zero;
use rand_distr::{StandardNormal, Distribution};

use crate::decimal::Decimal;

use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct BoundingFunctionMahalanobisDistance
{
    mu_sigma: Vec<Vec<(f64, f64)>>,
    #[serde(skip)]
    d: Vec<Vec<f64>>,
    #[serde(skip)]
    bias: f64
}

impl BoundingFunctionMahalanobisDistance
{
    pub fn new<
        const SIZES: &'static [usize],
        const MU_MEAN: Decimal<NEURON_PARAMETER_DECIMALS, i16>,
        const MU_VARIANCE: Decimal<NEURON_PARAMETER_DECIMALS, u16>,
        const SIGMA_MEAN: Decimal<NEURON_PARAMETER_DECIMALS, i16>,
        const SIGMA_VARIANCE: Decimal<NEURON_PARAMETER_DECIMALS, u16>,
        O
    >() -> O
    where Self: Into<O>
    {
        let rng = &mut rand::thread_rng();
        let mu_mean: f64 = MU_MEAN.into();
        let mu_variance: f64 = MU_VARIANCE.into();
        let sigma_mean: f64 = SIGMA_MEAN.into();
        let sigma_variance: f64 = SIGMA_VARIANCE.into();
        BoundingFunctionMahalanobisDistance {
            mu_sigma: SIZES.iter()
                .map(|size| (0..*size)
                    .map(|_| (
                        mu_mean + mu_variance*<StandardNormal as Distribution<f64>>::sample(&StandardNormal, rng),
                        loop
                        {
                            let sigma = sigma_mean + sigma_variance*<StandardNormal as Distribution<f64>>::sample(&StandardNormal, rng);
                            if sigma >= 0.0
                            {
                                break sigma;
                            }
                        }
                    )).collect()
                ).collect(),
            d: vec![],
            bias: 0.0
        }.into()
    }
}

#[typetag::serde(name = "MahalanobisDistance")]
#[clone_box]
impl BoundingFunctionAny for BoundingFunctionMahalanobisDistance
{
    
}

#[clone_box]
impl<T> BoundingFunction<f64, T> for BoundingFunctionMahalanobisDistance
where
    f64: NeuronState<T> + NeuronState<<T as NeuronStateDerivateable<f64>>::Slope>,
    T: NeuronStateDerivateable<f64>,
    <T as NeuronStateDerivateable<f64>>::Slope: Clone
{
    fn bound(self: &Self, x: f64) -> T
    {
        x.into_state()
    }
    fn slope(self: &Self, _x: f64) -> Option<<T as NeuronStateDerivateable<f64>>::Slope>
    {
        None
    }
    fn slope_for(self: &Self, x: f64, index: Option<(usize, usize)>) -> <T as NeuronStateDerivateable<f64>>::Slope
    {
        let sigma = match index
        {
            Some(index) => match self.mu_sigma.get(index.0).and_then(|d| d.get(index.1))
            {
                Some(&(_, sigma)) => sigma,
                None => 1.0
            }
            None => 1.0
        };
        if !sigma.is_zero()
        {
            let d = match index
            {
                Some(index) => match self.d.get(index.0).and_then(|d| d.get(index.1))
                {
                    Some(&d) => d,
                    None => 0.0
                }
                None => self.bias
            };
            let x_abs = x.abs();
            d*x_abs*x_abs*x_abs
        }
        else
        {
            0.0
        }.max(f64::MIN).min(f64::MAX).into_state()
    }
    fn slope_params(self: &Self, x: f64) -> Vec<<T as NeuronStateDerivateable<f64>>::Slope>
    {
        let x_abs = x.abs();
        let x_abs3 = x_abs*x_abs*x_abs;
        let mu: Vec<<T as NeuronStateDerivateable<f64>>::Slope> = self.d.iter()
            .flatten()
            .zip(self.mu_sigma.iter().flatten())
            .map(|(&d, &(_, sigma))| if !sigma.is_zero()
            {
                -d*x_abs3
            }
            else
            {
                0.0
            }.max(f64::MIN).min(f64::MAX).into_state())
            .collect();
        let sigma: Vec<<T as NeuronStateDerivateable<f64>>::Slope> = self.d.iter()
            .flatten()
            .zip(self.mu_sigma.iter().flatten())
            .map(|(&d, &(_, sigma))| if !sigma.is_zero()
            {
                -0.5*d*d*x_abs3
            }
            else
            {
                d.signum()*sigma.signum()*INFINITY
            }.max(f64::MIN).min(f64::MAX).into_state())
            .collect();
        [mu, sigma].concat()
    }
    fn accumulate(self: &mut Self, inputs: &[Vec<f64>], bias: f64) -> Option<f64>
    {
        self.bias = bias;
        self.d = inputs.into_iter()
            .zip(self.mu_sigma.iter())
            .map(|(inputs, mu_sigma)| inputs.into_iter()
                    .zip(mu_sigma.into_iter())
                    .map(|(&x, &(mu, sigma))| (x - mu)/sigma)
                    .collect()
            ).collect();
        let dist2: f64 = bias*bias.abs() + self.d.clone().into_iter()
            .zip(self.mu_sigma.iter())
            .flat_map(|(d, mu_sigma)| d.into_iter()
                .zip(mu_sigma.into_iter())
                .map(|(d, &(_, sigma))| d*sigma.abs().sqrt())
                .collect::<Vec<f64>>()
            ).sum::<f64>();
        Some((dist2.abs().sqrt().recip()*dist2.signum()).into_state())
    }
}