#![cfg_attr(not(test), no_std)]

mod decimator;
mod differentiator;
mod integrator;

/// A cic filter
/// M: decimation factor
/// N: number of stages
pub struct CicFilter<const M: usize, const N: usize> {
    decimator: decimator::Decimator<M>,
    integrators: [integrator::Integrator; N],
    differentiators: [differentiator::Differentiator; N],
}

impl<const M: usize, const N: usize> CicFilter<M, N> {
    fn new() -> Self {
        Self {
            decimator: decimator::Decimator::new(),
            integrators: [integrator::Integrator::new(); N],
            differentiators: [differentiator::Differentiator::new(); N],
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
