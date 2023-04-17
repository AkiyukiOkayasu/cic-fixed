#![cfg_attr(not(test), no_std)]

/// A cic filter
/// M: decimation factor
/// N: number of stages
pub struct CicDecimationFilter<const M: usize, const N: usize> {
    decimator_counter: usize,
    integrators_last_output: [i32; N],
    differentiators_last_input: [i32; N],
}

impl<const M: usize, const N: usize> CicDecimationFilter<M, N> {
    pub fn new() -> Self {
        Self {
            decimator_counter: 0,
            integrators_last_output: [0i32; N],
            differentiators_last_input: [0i32; N],
        }
    }

    #[inline]
    pub fn filter(&mut self, input: i32) -> Option<i32> {
        let mut output = input;
        for e in self.integrators_last_output.iter_mut() {
            // integrator
            output = e.wrapping_add(output);
            *e = output;
        }

        self.decimator_counter += 1;

        if self.decimator_counter == M {
            self.decimator_counter = 0;
            for d in self.differentiators_last_input.iter_mut() {
                // differentiator
                let temp = output;
                output = output.wrapping_sub(*d);
                *d = temp;
            }
            Some(output)
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
