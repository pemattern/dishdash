use lib::GenerateSchemas;

#[derive(GenerateSchemas)]
#[schema(read_only)]
pub struct Unit {
    pub code: String,
    pub physical_quantity_code: String,
    pub unit_system_code: String,
    pub symbol: String,
    pub si_factor: f64,
}
