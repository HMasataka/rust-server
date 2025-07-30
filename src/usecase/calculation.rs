use crate::domain::entities::calculation::Calculation;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CalculationRequest {
    pub operand_a: i32,
    pub operand_b: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CalculationResponse {
    pub result: i32,
    pub operation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HelloResponse {
    pub message: String,
    pub sample_calculations: Vec<CalculationResponse>,
}

pub struct CalculationUseCase;

impl CalculationUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn add(&self, request: CalculationRequest) -> Result<CalculationResponse, String> {
        let calculation =
            Calculation::new(request.operand_a, request.operand_b).map_err(|e| e.to_string())?;

        let result = calculation.add();
        Ok(CalculationResponse {
            result,
            operation: "addition".to_string(),
        })
    }

    pub fn subtract(&self, request: CalculationRequest) -> Result<CalculationResponse, String> {
        let calculation =
            Calculation::new(request.operand_a, request.operand_b).map_err(|e| e.to_string())?;

        let result = calculation.subtract();
        Ok(CalculationResponse {
            result,
            operation: "subtraction".to_string(),
        })
    }

    pub fn multiply(&self, request: CalculationRequest) -> Result<CalculationResponse, String> {
        let calculation =
            Calculation::new(request.operand_a, request.operand_b).map_err(|e| e.to_string())?;

        let result = calculation.multiply();
        Ok(CalculationResponse {
            result,
            operation: "multiplication".to_string(),
        })
    }

    pub fn divide(&self, request: CalculationRequest) -> Result<CalculationResponse, String> {
        let calculation =
            Calculation::new(request.operand_a, request.operand_b).map_err(|e| e.to_string())?;

        let result = calculation.divide().map_err(|e| e.to_string())?;

        Ok(CalculationResponse {
            result,
            operation: "division".to_string(),
        })
    }

    pub fn get_hello_with_calculations(&self) -> Result<HelloResponse, String> {
        let request = CalculationRequest {
            operand_a: 10,
            operand_b: 5,
        };

        let mut calculations = Vec::new();
        calculations.push(self.add(request.clone())?);
        calculations.push(self.subtract(request.clone())?);
        calculations.push(self.multiply(request.clone())?);
        calculations.push(self.divide(request.clone())?);

        Ok(HelloResponse {
            message: "Hello World with Onion Architecture!".to_string(),
            sample_calculations: calculations,
        })
    }
}
