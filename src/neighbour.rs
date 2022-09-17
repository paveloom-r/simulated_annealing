//! Provides the [`NeighbourMethod`](crate::NeighbourMethod) enum

use anyhow::{Context, Result};
use itertools::izip;
use num::Float;
use rand::prelude::*;
use rand_distr::{Normal, StandardNormal};

use std::fmt::Debug;

use crate::{utils, Bounds, Point};

/// Method of getting a random neighbour
pub enum Method<F, R, const N: usize>
where
    F: Float,
    StandardNormal: Distribution<F>,
    R: Rng,
{
    /// Get a neighbour in the vicinity of the current point
    /// by sampling a random normal distribution with the mean
    /// in that point and with the provided standard deviation
    Normal {
        /// Standard deviation
        sd: F,
    },
    /// Custom: choose your own!
    #[allow(clippy::complexity)]
    Custom {
        /// Custom function
        f: fn(p: &Point<F, N>, bounds: &Bounds<F, N>, rng: &mut R) -> Result<Point<F, N>>,
    },
}

impl<F, R, const N: usize> Method<F, R, N>
where
    F: Float + Debug,
    StandardNormal: Distribution<F>,
    R: Rng,
{
    /// Get a neighbour of the current point
    ///
    /// # Arguments
    /// * `p` --- Current point;
    /// * `bounds` --- Bounds of the parameter space;
    /// * `distribution` --- Distribution to sample from;
    /// * `rng` --- Random number generator.
    ///
    /// # Errors
    ///
    /// Will return `Err` if
    /// * Normal: `sd` is not finite
    /// * Custom function returned `Err`
    pub fn neighbour(
        &self,
        p: &Point<F, N>,
        bounds: &Bounds<F, N>,
        rng: &mut R,
    ) -> Result<Point<F, N>> {
        match *self {
            Method::Normal { sd } => {
                let mut new_p = [F::zero(); N];
                // Generate a new point
                izip!(&mut new_p, p, bounds)
                    .try_for_each(|(new_c, &c, r)| -> Result<()> {
                        // Create a normal distribution around the current coordinate
                        let d = Normal::new(c, sd)
                            .with_context(|| "Couldn't create a normal distribution")?;
                        // Sample from this distribution
                        let mut s = d.sample(rng);
                        // If the result is not in the range, repeat until it is
                        while !r.contains(&s) {
                            s = d.sample(rng);
                        }
                        // Save the new coordinate
                        *new_c = utils::cast(s)?;
                        Ok(())
                    })
                    .with_context(|| "Couldn't generate a new point")?;
                Ok(new_p)
            }
            Method::Custom { f } => f(p, bounds, rng),
        }
    }
}
