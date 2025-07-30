use crate::domain::entities::calculation::Calculation;

pub trait CalculationRepository {
    fn save(&self, calculation: &Calculation) -> Result<(), String>;
    fn find_by_id(&self, id: u64) -> Result<Option<Calculation>, String>;
}

