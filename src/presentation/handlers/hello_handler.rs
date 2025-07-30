use axum::Json;
use crate::application::services::CalculationService;
use crate::application::dto::calculation_dto::HelloResponse;
use crate::presentation::controllers::HelloController;

#[utoipa::path(
    get,
    path = "/",
    tag = "Hello World",
    responses(
        (status = 200, description = "Hello World with calculations", body = HelloResponse),
    ),
)]
pub async fn hello_handler() -> Result<Json<HelloResponse>, String> {
    let calculation_service = CalculationService::new();
    let controller = HelloController::new(calculation_service);
    
    match controller.get_hello() {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(err),
    }
}