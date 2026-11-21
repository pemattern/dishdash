use lib::Payload;

#[derive(Payload)]
#[payload(read_only)]
struct UnitPayload {
    pub code: String,
    pub physical_quantity_code: String,
    pub unit_system_code: String,
    pub symbol: String,
    pub si_factor: f64,
}
