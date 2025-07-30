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

