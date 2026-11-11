use lib::GenerateSchemas;

#[derive(GenerateSchemas)]
#[schema(read_only)]
pub struct UnitSystem {
    pub code: String,
}
