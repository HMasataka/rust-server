#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number {
    value: i32,
}

impl Number {
    pub fn new(value: i32) -> Result<Self, &'static str> {
        if value < -1_000_000 || value > 1_000_000 {
            Err("Number out of range")
        } else {
            Ok(Self { value })
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn is_even(&self) -> bool {
        self.value % 2 == 0
    }

    pub fn format(&self) -> String {
        format!("Number: {}", self.value)
    }
}