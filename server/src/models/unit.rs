use crate::requests::constants::{PhysicalQuantity, UnitSystem};

pub struct UnitModel {
    pub code: String,
    pub physical_quantity: PhysicalQuantity,
    pub unit_system: UnitSystem,
    pub symbol: String,
    pub si_factor: f64,
}
