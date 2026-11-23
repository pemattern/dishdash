use crate::requests::constants::{PhysicalQuantityCode, UnitCode, UnitSystemCode};

pub struct UnitModel {
    pub code: UnitCode,
    pub physical_quantity_code: PhysicalQuantityCode,
    pub unit_system_code: UnitSystemCode,
    pub symbol: String,
    pub si_factor: f64,
}
