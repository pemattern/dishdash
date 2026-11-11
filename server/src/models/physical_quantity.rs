use lib::GenerateSchemas;

#[derive(GenerateSchemas)]
#[schema(read_only)]
pub struct PhysicalQuantity {
    pub code: String,
}
