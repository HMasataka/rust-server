use crate::application::services::CalculationService;
use crate::application::dto::calculation_dto::HelloResponse;

pub struct HelloController {
    calculation_service: CalculationService,
}

impl HelloController {
    pub fn new(calculation_service: CalculationService) -> Self {
        Self {
            calculation_service,
        }
    }

    pub fn get_hello(&self) -> Result<HelloResponse, String> {
        self.calculation_service.get_hello_with_calculations()
    }
}