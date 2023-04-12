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
    pub fn new() -> Self {
        Self {
            decimator: decimator::Decimator::new(),
            integrators: [integrator::Integrator::new(); N],
            differentiators: [differentiator::Differentiator::new(); N],
        }
    }

    pub fn filter(&mut self, input: i32) -> Option<i32> {
        let mut output = input;
        for integrator in self.integrators.iter_mut() {
            output = integrator.integrate(output);
        }

        if let Some(output) = self.decimator.decimate(output) {
            let mut v = output;
            for differentiator in self.differentiators.iter_mut() {
                v = differentiator.differentiate(v);
            }
            Some(v)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut filter = CicFilter::<4, 2>::new();
        let result = filter.filter(0);
        assert!(result.is_none());
        let result = filter.filter(1);
        assert!(result.is_none());
        let result = filter.filter(2);
        assert!(result.is_none());
        let result = filter.filter(3);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 10);

        let result = filter.filter(2);
        assert!(result.is_none());
        let result = filter.filter(-1);
        assert!(result.is_none());
        let result = filter.filter(-2);
        assert!(result.is_none());
        let result = filter.filter(1);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 16);
    }
}
