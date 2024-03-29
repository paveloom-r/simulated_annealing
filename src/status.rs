//! Provides the [`Status`](crate::Status) enum

use num::Float;

use core::fmt::Debug;

/// Custom status function
///
/// See why it's a `Box` [here](https://stackoverflow.com/a/59035722).
///
/// See the [`print`](Status#method.print) method for the signature explanation.
pub type Custom<'a, F, const N: usize> = Box<dyn FnMut(usize, F, F, [F; N], F, [F; N]) + 'a>;

/// Status function
pub enum Status<'a, F: Float + Debug, const N: usize> {
    /// Don't print status
    None,
    /// Print status when `k` is divisable by `nk`
    Periodic {
        /// A number of iterations between calls
        nk: usize,
    },
    /// Custom: choose your own!
    Custom {
        /// Custom function
        f: Custom<'a, F, N>,
    },
}

impl<'a, F: Float + Debug, const N: usize> Status<'a, F, N> {
    /// Print the status
    ///
    /// # Arguments
    /// * `k` --- Current iteration;
    /// * `t` --- Current temperature;
    /// * `f` --- Current solution;
    /// * `p` --- Current point;
    /// * `best_f` --- Current best solution;
    /// * `best_p` --- Current point of the best solution.
    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::print_stdout)]
    #[allow(clippy::use_debug)]
    pub fn print(&mut self, k: usize, t: F, f: F, p: [F; N], best_f: F, best_p: [F; N]) {
        match *self {
            Status::None => (),
            Status::Periodic { nk } => {
                if k % nk == 0 {
                    println!(
                        "k: {k}\nt: {t:#?}:\ncurrent: {f:#?} at {p:#?}\nbest: {best_f:#?} at {best_p:#?}\n"
                    );
                }
            }
            Status::Custom { f: ref mut fun } => fun(k, t, f, p, best_f, best_p),
        }
    }
}
