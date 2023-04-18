//! Decimator (downsampler) for cic decimation filter.  

/// Decimator
#[derive(Default, Clone, Copy)]
pub(crate) struct Decimator<const M: usize> {
    counter: usize,
}

impl<const M: usize> Decimator<M> {
    pub(crate) fn new() -> Self {
        Self { counter: 0 }
    }

    /// Decimate the input and return the output.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to decimate.
    ///
    /// # Returns
    ///
    /// The output of the decimator.  
    /// When the decimator is ready to output a value, it will return some(input). Otherwise, it will return None.  
    #[inline]
    pub(crate) fn decimate(&mut self, input: i32) -> Option<i32> {
        self.counter += 1;
        if self.counter == M {
            self.counter = 0;
            Some(input)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimator_test() {
        let mut decimator = Decimator::<4>::new();
        let result = decimator.decimate(1);
        assert!(result.is_none());
        let result = decimator.decimate(2);
        assert!(result.is_none());
        let result = decimator.decimate(3);
        assert!(result.is_none());
        let result = decimator.decimate(4);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 4);

        let result = decimator.decimate(1);
        assert!(result.is_none());
        let result = decimator.decimate(2);
        assert!(result.is_none());
        let result = decimator.decimate(3);
        assert!(result.is_none());
        let result = decimator.decimate(4);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 4);
    }
}
