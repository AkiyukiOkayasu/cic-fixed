#[derive(Default, Clone, Copy)]
pub(crate) struct Differentiator {
    last_input: i32,
}

impl Differentiator {
    pub(crate) fn new() -> Self {
        Self { last_input: 0 }
    }

    pub(crate) fn differentiate(&mut self, input: i32) -> i32 {
        let output = input.wrapping_sub(self.last_input);
        self.last_input = input;
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn differentiate_test() {
        let mut differentiator = Differentiator::new();
        let result = differentiator.differentiate(2);
        assert_eq!(result, 2);
        let result = differentiator.differentiate(5);
        assert_eq!(result, 3);
    }
}
