#[derive(Default, Clone, Copy)]
pub(crate) struct Integrator {
    last_output: i32,
}

impl Integrator {
    pub(crate) fn new() -> Self {
        Self { last_output: 0 }
    }

    pub(crate) fn integrate(&mut self, input: i32) -> i32 {
        let output = self.last_output.wrapping_add(input);
        self.last_output = output;
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrate_test() {
        let mut integrator = Integrator::new();
        let result = integrator.integrate(2);
        assert_eq!(result, 2);
        let result = integrator.integrate(5);
        assert_eq!(result, 7);
        let result = integrator.integrate(10);
        assert_eq!(result, 17);
    }
}
