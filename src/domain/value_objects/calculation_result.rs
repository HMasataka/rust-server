#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalculationResult {
    value: i32,
}

impl CalculationResult {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn display(&self) -> String {
        self.value.to_string()
    }
}