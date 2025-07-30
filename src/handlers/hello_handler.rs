use crate::usecase::calculation::CalculationUseCase;
use crate::usecase::calculation::HelloResponse;
use axum::Json;

#[utoipa::path(
    get,
    path = "/",
    tag = "Hello World",
    responses(
        (status = 200, description = "Hello World with calculations", body = HelloResponse),
    ),
)]
pub async fn hello_handler() -> Result<Json<HelloResponse>, String> {
    let calculation_service = CalculationUseCase::new();

    match calculation_service.get_hello_with_calculations() {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(err),
    }
}
