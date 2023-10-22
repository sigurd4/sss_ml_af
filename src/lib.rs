#![feature(decl_macro)]

#![feature(generic_const_exprs)]

moddef::moddef!(
    flat(pub) mod {
        activation_function
    },
    mod {
        plot for cfg(test)
    }
);

use array_math::*;

#[cfg(test)]
mod tests
{
    use std::fmt::Debug;

    use linspace::LinspaceArray;

    use super::*;

    const N: usize = 100;

    pub fn plot_bf<BF, R, const K: usize>(bf: BF, z: R, k: [f64; K])
    where
        BF: BoundingFunction<f64, K> + Debug,
        R: LinspaceArray<f64, N>
    {
        let z = z.linspace_array();
        let y = z.map2(|z| bf.bf_y(z, k));
        let dydz = z.map2(|z| bf.bf_dydz(z, k).0);

        let bf_name = format!("{:?}", bf);

        let mut first = true;
        let file_name: String = bf_name.chars()
            .flat_map(|c| {
                let c_low = c.to_ascii_lowercase();
                if c.is_uppercase() && !first
                {
                    vec!['_', c_low]
                }
                else
                {
                    first = false;
                    vec![c_low]
                }
            }).collect();

        crate::plot::plot_curve(&format!("{}: y(z)", bf_name), &format!("plot/af_{}_y.png", &file_name), z, y)
            .expect("Failed plot");
        crate::plot::plot_curve(&format!("{}: dy/dz(z)", bf_name), &format!("plot/af_{}_dydz.png", &file_name), z, y)
            .expect("Failed plot");
    }
}

macro f
{
    ($n:expr; $f:ident) => {
        <$f>::from($n).unwrap()
    },
    ($n:expr) => {
        num::NumCast::from($n).unwrap()
    },
}