#[derive(Debug, Clone)]
pub struct Calculation {
    operand_a: i32,
    operand_b: i32,
}

impl Calculation {
    pub fn new(a: i32, b: i32) -> Result<Self, &'static str> {
        Ok(Self {
            operand_a: a,
            operand_b: b,
        })
    }

    pub fn add(&self) -> i32 {
        self.operand_a + self.operand_b
    }

    pub fn subtract(&self) -> i32 {
        self.operand_a - self.operand_b
    }

    pub fn multiply(&self) -> i32 {
        self.operand_a * self.operand_b
    }

    pub fn divide(&self) -> Result<i32, &'static str> {
        if self.operand_b == 0 {
            Err("Division by zero")
        } else {
            Ok(self.operand_a / self.operand_b)
        }
    }

    pub fn operand_a(&self) -> &i32 {
        &self.operand_a
    }

    pub fn operand_b(&self) -> &i32 {
        &self.operand_b
    }
}
