use rand_distr::{StandardNormal, Distribution};

use crate::decimal::Decimal;

use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct BoundingFunctionInvEuclidianDistance
{
    mu: Vec<Vec<f64>>,
    #[serde(skip)]
    d: Vec<Vec<f64>>,
    #[serde(skip)]
    bias: f64
}

impl BoundingFunctionInvEuclidianDistance
{
    pub fn new<
        const SIZES: &'static [usize],
        const MU_MEAN: Decimal<NEURON_PARAMETER_DECIMALS, i16>,
        const MU_VARIANCE: Decimal<NEURON_PARAMETER_DECIMALS, u16>,
        O
    >() -> O
    where Self: Into<O>
    {
        let rng = &mut rand::thread_rng();
        let mu_mean: f64 = MU_MEAN.into();
        let mu_variance: f64 = MU_VARIANCE.into();
        BoundingFunctionInvEuclidianDistance {
            mu: SIZES.iter()
                .map(|size| (0..*size)
                    .map(|_| mu_mean + mu_variance*<StandardNormal as Distribution<f64>>::sample(&StandardNormal, rng))
                    .collect()
                ).collect(),
            d: vec![],
            bias: 0.0
        }.into()
    }
}

#[typetag::serde(name = "InvEuclidianDistance")]
#[clone_box]
impl BoundingFunctionAny for BoundingFunctionInvEuclidianDistance
{
    
}

#[clone_box]
impl<T> BoundingFunction<f64, T> for BoundingFunctionInvEuclidianDistance
where
    f64: NeuronState<T> + NeuronState<<T as NeuronStateDerivateable<f64>>::Slope>,
    T: NeuronStateDerivateable<f64>,
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
        let d = match index
        {
            Some(index) => match self.d.get(index.0).and_then(|d| d.get(index.1))
            {
                Some(&d) => d,
                None => 0.0
            }
            None => self.bias
        };
        (-d*x*x*x).into_state()
    }
    fn slope_params(self: &Self, x: f64) -> Vec<<T as NeuronStateDerivateable<f64>>::Slope>
    {
        let x3 = x*x*x;
        self.d.iter()
            .flatten()
            .map(|&d| (d*x3).into_state())
            .collect()
    }
    fn accumulate(self: &mut Self, inputs: &[Vec<f64>], bias: f64) -> Option<f64>
    {
        self.bias = bias;
        self.d = inputs.into_iter()
            .enumerate()
            .map(|(l, inputs)| match self.mu.get(l)
            {
                Some(mu) => inputs.into_iter()
                    .enumerate()
                    .map(|(i, &x)| match mu.get(i)
                    {
                        Some(mu) => x - mu,
                        None => x
                    }).collect(),
                None => inputs.clone()
            }).collect();
        let dist2: f64 = bias*bias + self.d.clone().into_iter()
            .flatten()
            .map(|d| d*d)
            .sum::<f64>();
        Some(dist2.sqrt().recip().into_state())
    }
}