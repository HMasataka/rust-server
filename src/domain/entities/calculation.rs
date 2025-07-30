use crate::domain::value_objects::{CalculationResult, Number};

#[derive(Debug, Clone)]
pub struct Calculation {
    operand_a: Number,
    operand_b: Number,
}

impl Calculation {
    pub fn new(a: i32, b: i32) -> Result<Self, &'static str> {
        Ok(Self {
            operand_a: Number::new(a)?,
            operand_b: Number::new(b)?,
        })
    }

    pub fn add(&self) -> CalculationResult {
        CalculationResult::new(self.operand_a.value() + self.operand_b.value())
    }

    pub fn subtract(&self) -> CalculationResult {
        CalculationResult::new(self.operand_a.value() - self.operand_b.value())
    }

    pub fn multiply(&self) -> CalculationResult {
        CalculationResult::new(self.operand_a.value() * self.operand_b.value())
    }

    pub fn divide(&self) -> Result<CalculationResult, &'static str> {
        if self.operand_b.value() == 0 {
            Err("Division by zero")
        } else {
            Ok(CalculationResult::new(
                self.operand_a.value() / self.operand_b.value(),
            ))
        }
    }

    pub fn operand_a(&self) -> &Number {
        &self.operand_a
    }

    pub fn operand_b(&self) -> &Number {
        &self.operand_b
    }
}

