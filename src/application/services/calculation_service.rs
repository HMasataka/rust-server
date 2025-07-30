use crate::domain::entities::calculation::Calculation;
use crate::application::dto::calculation_dto::{CalculationRequest, CalculationResponse, HelloResponse};

pub struct CalculationService;

impl CalculationService {
    pub fn new() -> Self {
        Self
    }

    pub fn add(&self, request: CalculationRequest) -> Result<CalculationResponse, String> {
        let calculation = Calculation::new(request.operand_a, request.operand_b)
            .map_err(|e| e.to_string())?;
        
        let result = calculation.add();
        Ok(CalculationResponse {
            result: result.value(),
            operation: "addition".to_string(),
        })
    }

    pub fn subtract(&self, request: CalculationRequest) -> Result<CalculationResponse, String> {
        let calculation = Calculation::new(request.operand_a, request.operand_b)
            .map_err(|e| e.to_string())?;
        
        let result = calculation.subtract();
        Ok(CalculationResponse {
            result: result.value(),
            operation: "subtraction".to_string(),
        })
    }

    pub fn multiply(&self, request: CalculationRequest) -> Result<CalculationResponse, String> {
        let calculation = Calculation::new(request.operand_a, request.operand_b)
            .map_err(|e| e.to_string())?;
        
        let result = calculation.multiply();
        Ok(CalculationResponse {
            result: result.value(),
            operation: "multiplication".to_string(),
        })
    }

    pub fn divide(&self, request: CalculationRequest) -> Result<CalculationResponse, String> {
        let calculation = Calculation::new(request.operand_a, request.operand_b)
            .map_err(|e| e.to_string())?;
        
        let result = calculation.divide()
            .map_err(|e| e.to_string())?;
        
        Ok(CalculationResponse {
            result: result.value(),
            operation: "division".to_string(),
        })
    }

    pub fn get_hello_with_calculations(&self) -> Result<HelloResponse, String> {
        let request = CalculationRequest { operand_a: 10, operand_b: 5 };
        
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