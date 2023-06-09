//! # cic-fixed
//!
//! A CIC filter implementation for fixed point numbers.  
//! Implemented for use in converting PDM to PCM.  
//!
//! ## Example
//!
//! ```rust
//! use cic_fixed::CicDecimationFilter;
//!
//! let mut filter = CicDecimationFilter::<4, 2>::new();
//! let result = filter.filter(0);
//! assert!(result.is_none());
//! let result = filter.filter(1);
//! assert!(result.is_none());
//! let result = filter.filter(2);
//! assert!(result.is_none());
//! let result = filter.filter(3);
//! assert!(result.is_some());
//! assert_eq!(result.unwrap(), 10);
//! ```
//!
#![cfg_attr(not(test), no_std)]

mod decimator;
mod differentiator;
mod integrator;

/// CIC decimation filter.  
/// - `M` - Decimation factor  
/// - `N` - Number of stages  
pub struct CicDecimationFilter<const M: usize, const N: usize> {
    decimator: decimator::Decimator<M>,
    integrators: [integrator::Integrator; N],
    differentiators: [differentiator::Differentiator; N],
}

impl<const M: usize, const N: usize> CicDecimationFilter<M, N> {
    pub fn new() -> Self {
        Self {
            decimator: decimator::Decimator::new(),
            integrators: [integrator::Integrator::new(); N],
            differentiators: [differentiator::Differentiator::new(); N],
        }
    }

    /// Process the input and return the output when the decimator is ready to output a value.     
    ///
    /// # Arguments
    ///
    /// * `input` - The input to filter.
    ///
    /// # Returns
    ///
    /// The output of the filter.  
    /// When the decimator is ready to output a value, it will return some(input). Otherwise, it will return None.  
    #[inline]
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
            // TODO 出力はM^Nで割らないといけないかも
            Some(v)
        } else {
            None
        }
    }
}

impl<const M: usize, const N: usize> Default for CicDecimationFilter<M, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cic_decimation_test() {
        let mut filter = CicDecimationFilter::<4, 2>::new();
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

        let result = filter.filter(2);
        assert!(result.is_none());
        let result = filter.filter(-1);
        assert!(result.is_none());
        let result = filter.filter(-2);
        assert!(result.is_none());
        let result = filter.filter(1);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 0);

        let result = filter.filter(2);
        assert!(result.is_none());
        let result = filter.filter(-1);
        assert!(result.is_none());
        let result = filter.filter(-2);
        assert!(result.is_none());
        let result = filter.filter(1);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 0);

        let result = filter.filter(0);
        assert!(result.is_none());
        let result = filter.filter(1);
        assert!(result.is_none());
        let result = filter.filter(2);
        assert!(result.is_none());
        let result = filter.filter(3);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 8);

        let result = filter.filter(3);
        assert!(result.is_none());
        let result = filter.filter(3);
        assert!(result.is_none());
        let result = filter.filter(-2);
        assert!(result.is_none());
        let result = filter.filter(1);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 32);
    }

    #[test]
    fn overflow_test() {
        let mut filter = CicDecimationFilter::<4, 2>::new();

        for _ in 0..1000 {
            filter.filter(i32::MAX);
        }
    }
}
